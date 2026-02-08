<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useHistoryStore } from '../stores/history'
import { useSettingsStore } from '../stores/settings'
import { useRouter } from 'vue-router'
import { openUrl } from '@tauri-apps/plugin-opener'
import { LINKS } from '../config/links'
import { useSafeAuthStore, getSafeAIFormatService } from '../auth/authWrapper'
import { pandocService, TemplateInfo, TemplateMeta, ConvertOptions } from '../services/pandocService'
import { buildPandocMetadata, mergeConfigs } from '../services/configTransform'
import { saveRecentConfig } from '../services/configStorage'
import { DEFAULT_CONFIG } from '../types/templateConfig'
import type { TemplateConfig, ConfigPreset } from '../types/templateConfig'
import TemplateConfigDialog from '../components/TemplateConfigDialog.vue'
import PresetDialog from '../components/PresetDialog.vue'

const uploadStore = useUploadStore()
const historyStore = useHistoryStore()
const settingsStore = useSettingsStore()
const router = useRouter()
const authEnabled = import.meta.env.VITE_ENABLE_AUTH === 'true'

// Auth store and ultra member check
let authStore: any = null
const isUltraMember = ref(false)

const templates = ref<TemplateMeta[]>([])
const selectedTemplate = ref<TemplateMeta | null>(null)
const isLoading = ref(false)
const loadingMessage = ref('转换中...')
const error = ref('')
const hasPremium = ref(true)
const useAIFix = ref(false) // AI format fix checkbox

// 用户配置
const userConfig = ref<Partial<TemplateConfig>>({})

// 配置对话框状态
const configDialogVisible = ref(false)

// 预设对话框状态
const presetDialogVisible = ref(false)
const presetDialogMode = ref<'load' | 'save'>('load')

onMounted(async () => {
  if (!uploadStore.preparedInput) {
    router.push('/upload')
    return
  }
  
  // Initialize auth store
  authStore = useSafeAuthStore()
  if (authEnabled) {
      // Check if user has active ultra membership
      const membership = authStore.activeMembership
      isUltraMember.value = membership?.membershipType === 'ultra'
  }
  
  try {
    const response = await pandocService.getTemplates()
    console.log('Template list fetched:', response) // Debug log
    templates.value = response.templates
    hasPremium.value = response.has_premium
    selectedTemplate.value = templates.value[0] || null

    // 初始化配置
    initConfig()
  } catch (e) {
    console.error('Error fetching template list:', e) // Debug log
    error.value = '无法加载模板列表，请稍后重试'
  }
})

// 初始化配置
const initConfig = () => {
  if (selectedTemplate.value?.defaultPreset) {
    userConfig.value = { ...selectedTemplate.value.defaultPreset } as Partial<TemplateConfig>
  } else {
    userConfig.value = {}
  }
}

const selectTemplate = (template: TemplateMeta) => {
  selectedTemplate.value = template
  error.value = ''
  
  // 切换模板时重置配置为该模板的默认预设
  if (template.defaultPreset) {
    userConfig.value = { ...template.defaultPreset } as Partial<TemplateConfig>
  } else {
    userConfig.value = {}
  }
}

const convertMarkdown = async () => {
  if (!selectedTemplate.value) return

  try {
    isLoading.value = true
    error.value = ''
    
    const inputs = uploadStore.preparedInputs || (uploadStore.preparedInput ? [uploadStore.preparedInput] : [])
    if (inputs.length === 0) {
        error.value = '未找到预期输入文件'
        isLoading.value = false
        return
    }

    const total = inputs.length
    const results: any[] = []

    // Prepare template once if shared configuration
    // (Optimization: call prepareTemplate once since config is shared)
    // However, prepareTemplate returns a `reference_doc` path specific to the template copy?
    // Usually it unpacks the template. If unique per run, call inside loop. 
    // If idempotent/reusable, call outside. Safe to call once.
    
    loadingMessage.value = '模板准备中...'
    
    // Merge config once
    const finalConfig = mergeConfigs(
      userConfig.value,
      (selectedTemplate.value.defaultPreset as Partial<TemplateConfig>) || {},
      DEFAULT_CONFIG
    )
    const pandocMetadata = buildPandocMetadata(finalConfig)
    saveRecentConfig(finalConfig)

    // Process each file
    for (let i = 0; i < total; i++) {
        const input = inputs[i]
        const currentName = input.source_name || `File ${i+1}`
        loadingMessage.value = `(${i + 1}/${total}) 正在处理: ${currentName}`
        
        try {
            // Prepare template (Inside loop to prevent deletion issues)
            // Note: This might be slightly inefficient but safe if backend deletes files
             const templateInfo: TemplateInfo = await pandocService.prepareTemplate(
              selectedTemplate.value.id,
              selectedTemplate.value.member
            )

            // AI Fix
            if (useAIFix.value && authEnabled && isUltraMember.value && authStore) {
                 loadingMessage.value = `(${i + 1}/${total}) AI 格式修复: ${currentName}`
                 const aiService = getSafeAIFormatService()
                 if (aiService) {
                     await aiService.executeAIFormatFix(input.markdown_path, authStore)
                 }
            }
            
            loadingMessage.value = `(${i + 1}/${total}) 转换文档: ${currentName}`
            
            const convertOptions: ConvertOptions = {
              input_file: input.markdown_path,
              source_dir: input.source_dir,
              source_name: input.source_name,
              reference_doc: templateInfo.reference_doc,
              metadata: pandocMetadata,
              metadata_file: undefined,
              use_crossref: true
            }

            const outPath = await pandocService.convertMarkdown(convertOptions)
            
            results.push({
                fileName: currentName,
                outputPath: outPath,
                status: 'success'
            })
            
            // Log History (Success)
            try {
                const type = authStore?.activeMembership?.membershipType
                let max = 3
                if (type === 'ultra') max = 100
                else if (type === 'pro') max = 10
                const limit = Math.min(settingsStore.historyLimit, max)
                
                historyStore.addRecord({
                    id: Date.now().toString() + i,
                    date: Date.now(),
                    fileName: currentName,
                    templateName: selectedTemplate.value?.name || 'Unknown',
                    outputPath: outPath,
                    status: 'success'
                })
                historyStore.prune(limit)
            } catch (e) { console.error('History log failed', e) }

        } catch (currentError: any) {
            console.error(`File ${currentName} failed:`, currentError)
            results.push({
                fileName: currentName,
                status: 'failed',
                error: currentError.message || String(currentError)
            })
            
             // Log History (Fail)
            try {
                const type = authStore?.activeMembership?.membershipType
                let max = 3
                if (type === 'ultra') max = 100
                else if (type === 'pro') max = 10
                const limit = Math.min(settingsStore.historyLimit, max)
                
                historyStore.addRecord({
                    id: Date.now().toString() + i,
                    date: Date.now(),
                    fileName: currentName,
                    templateName: selectedTemplate.value?.name || 'Unknown',
                    status: 'failed',
                    errorMessage: currentError.message || String(currentError)
                })
                historyStore.prune(limit)
            } catch (e) { console.error('History log failed', e) }
        }
    }

    uploadStore.setResults(results)
    uploadStore.setStep(3)
    router.push('/result')

  } catch (err) {
    console.error('Batch process fatal error:', err)
    error.value = `处理中断: ${err instanceof Error ? err.message : String(err)}`
  } finally {
    isLoading.value = false
    loadingMessage.value = '转换中...'
  }
}

const goBack = () => {
  router.push('/upload')
}

// ... existing computed properties ...
const freeTemplates = computed(() => {
  return templates.value.filter((t) => !t.member)
})

const memberTemplates = computed(() => {
  if (!authEnabled) return []
  return templates.value.filter((t) => t.member)
})

const handleConfigConfirm = (config: Partial<TemplateConfig>) => { userConfig.value = config }
const handleConfigDialogLoadPreset = () => { configDialogVisible.value = false; presetDialogMode.value = 'load'; presetDialogVisible.value = true }
const handleConfigDialogSavePreset = () => { configDialogVisible.value = false; presetDialogMode.value = 'save'; presetDialogVisible.value = true }
const handleConfigDialogReset = () => { initConfig() }
const showConfigDialog = () => { configDialogVisible.value = true }

const handlePresetLoad = (config: Partial<TemplateConfig>) => { userConfig.value = { ...config }; configDialogVisible.value = true }
const handlePresetSave = (preset: ConfigPreset) => { console.log('预设已保存:', preset.name); configDialogVisible.value = true }
const handlePresetDialogClose = () => { presetDialogVisible.value = false }

const openDownloadPage = async () => { await openUrl(LINKS.releases) }
const isSelected = (template: TemplateMeta) => { return selectedTemplate.value?.id === template.id }
</script>

<template>
  <div class="p-6 pb-44 bg-[radial-gradient(circle_at_20%_20%,#f5f7ff,#eef2ff_40%,#e8edf8_80%)]">
    <!-- Progress Overlay -->
    <div v-if="isLoading" class="fixed inset-0 bg-black/40 flex items-center justify-center z-50 backdrop-blur-sm">
      <div class="bg-white rounded-2xl p-6 shadow-2xl max-w-sm w-11/12 text-center animate-fade-in">
        <div class="inline-block w-10 h-10 border-4 border-[#e0e7ff] border-t-[#6366f1] rounded-full animate-spin mb-4"></div>
        <h3 class="text-lg font-bold text-gray-900 m-0 mb-2">正在转换</h3>
        <p class="text-sm text-gray-600 m-0">{{ loadingMessage }}</p>
      </div>
    </div>

    <div class="max-w-6xl mx-auto mb-7 flex items-center justify-between gap-4">
      <div>
        <h1 class="m-0 text-[#1f2937] text-2xl tracking-tight">选择模板</h1>
        <p class="m-0 mt-1 text-[#4b5563] text-sm">选择一个适合你的文档模板，配置后开始转换</p>
      </div>
      <span class="bg-[#e0e7ff] text-[#3730a3] px-3 py-2 rounded-xl font-semibold text-xs whitespace-nowrap">Step 2 / 2</span>
    </div>

    <section class="max-w-6xl mx-auto mb-8">
      <h2 class="text-lg font-bold text-[#1f2937] m-0 mb-4">免费模板</h2>
      <div class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4">
        <div
          v-for="template in freeTemplates"
          :key="template.id"
          @click="selectTemplate(template)"
          class="bg-white border-2 border-[#e5e7eb] rounded-2xl p-3 cursor-pointer transition-all flex flex-col shadow-sm hover:border-[#c7d2fe] hover:shadow-[0_4px_16px_rgba(99,102,241,0.15)] hover:-translate-y-0.5"
          :class="[isSelected(template) ? 'border-[#7c3aed]! bg-[#faf5ff]! shadow-[0_4px_20px_rgba(124,58,237,0.2)]!' : '']"
        >
          <div v-if="isSelected(template)" class="h-3 flex justify-end -mb-3">
            <div class="text-[#16a34a] font-bold text-sm">✓ 已选择</div>
          </div>
          <div class="flex-1">
            <h3 class="m-0 mb-2 text-[#111827] text-lg font-bold flex items-center gap-2">
              <span
              class="text-xs bg-[#dcfce7] text-[#166534] px-2 py-1 rounded-md font-semibold">免费</span>
              {{ template.name }}
            </h3>
            <p class="m-0 text-[#6b7280] text-[13px] leading-relaxed">{{ template.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <section class="max-w-6xl mx-auto mb-8">
      <h2 class="text-lg font-bold text-[#1f2937] m-0 mb-4">会员模板</h2>
      <div v-if="memberTemplates.length > 0" class="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-4">
        <div
          v-for="template in memberTemplates"
          :key="template.id"
          @click="selectTemplate(template)"
          class="bg-white border-2 border-[#e5e7eb] rounded-2xl p-3 cursor-pointer transition-all flex flex-col shadow-sm hover:border-[#c7d2fe] hover:shadow-[0_4px_16px_rgba(99,102,241,0.15)] hover:-translate-y-0.5"
          :class="[isSelected(template) ? 'border-[#7c3aed]! bg-[#faf5ff]! shadow-[0_4px_20px_rgba(124,58,237,0.2)]!' : '']"
        >
          <div v-if="isSelected(template)" class="h-3 flex justify-end -mb-3">
            <div class="text-[#16a34a] font-bold text-sm">✓ 已选择</div>
          </div>
          <div class="flex-1">
            <h3 class="m-0 mb-2 text-[#111827] text-lg font-bold flex items-center gap-2">
              <span class="text-xs bg-[#ede9fe] text-[#6d28d9] px-2 py-1 rounded-md font-semibold">会员</span>
              {{ template.name }}
            </h3>
            <p class="m-0 text-[#6b7280] text-[13px] leading-relaxed">{{ template.description }}</p>
          </div>
        </div>
      </div>
      <div v-else-if="!authEnabled" class="bg-[linear-gradient(135deg,#f0fdfa,#ccfbf1)] border-2 border-dashed border-[#5eead4] rounded-2xl p-10 text-center text-[#0f766e]">
        <div class="text-5xl mb-3">🌟</div>
        <p class="m-0 font-semibold text-lg mb-2">这是开源版本，暂不支持高级模板</p>
        <p class="m-0 mt-1 text-[#14b8a6] text-sm mb-4">您可以下载完整版本解锁更多专业模板</p>
        <button 
          @click="openDownloadPage"
          class="inline-flex items-center gap-2 bg-[#0d9488] text-white px-5 py-2.5 rounded-lg font-semibold text-sm hover:bg-[#0f766e] transition-colors cursor-pointer border-0"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
          下载完整版本
        </button>
      </div>
      <div v-else class="bg-[linear-gradient(135deg,#f5f7ff,#ede9fe)] border-2 border-dashed border-[#c7d2fe] rounded-2xl p-10 text-center text-[#7c3aed]">
        <div class="text-5xl mb-3">🚀</div>
        <p class="m-0 font-semibold">敬请期待更多专业模板</p>
        <p class="m-0 mt-1 text-[#a78bfa] text-xs">登录后可解锁高级模板</p>
      </div>
    </section>

    <div v-if="error" class="max-w-6xl mx-auto mb-4 p-3.5 rounded-xl bg-[#fef2f2] text-[#b91c1c] border border-[#fecdd3]">{{ error }}</div>

    <!-- 配置对话框 -->
    <TemplateConfigDialog
      :visible="configDialogVisible"
      :config="userConfig"
      :template-preset="(selectedTemplate?.defaultPreset as Partial<TemplateConfig>)"
      @close="configDialogVisible = false"
      @confirm="handleConfigConfirm"
      @load-preset="handleConfigDialogLoadPreset"
      @save-preset="handleConfigDialogSavePreset"
      @reset="handleConfigDialogReset"
    />

    <!-- 预设对话框 -->
    <PresetDialog
      :visible="presetDialogVisible"
      :mode="presetDialogMode"
      :current-config="userConfig"
      @close="handlePresetDialogClose"
      @load="handlePresetLoad"
      @save="handlePresetSave"
    />

    <!-- AI Format Fix Bar -->
    <div v-if="isUltraMember" class="fixed left-0 right-0 bottom-18.25 bg-[#fffbeb] border-t border-[#fcd34d] z-10">
      <div class="max-w-6xl mx-auto px-6 py-2">
        <label class="inline-flex items-center gap-2 cursor-pointer text-[#92400e] text-xs font-semibold hover:text-[#b45309] transition-colors select-none">
          <div class="relative flex items-center justify-center w-4 h-4">
            <input 
              v-model="useAIFix" 
              type="checkbox" 
              class="peer appearance-none w-4 h-4 border-2 border-[#f59e0b] rounded bg-white checked:bg-[#f59e0b] checked:border-[#f59e0b] cursor-pointer transition-all"
            />
            <svg class="absolute w-2.5 h-2.5 text-white pointer-events-none opacity-0 peer-checked:opacity-100 transition-opacity" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="4" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
          </div>
          <div class="flex items-center gap-1.5">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="text-[#f59e0b]">
              <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
              <path d="M2 17l10 5 10-5"></path>
              <path d="M2 12l10 5 10-5"></path>
            </svg>
            <span>AI 格式修复</span>
          </div>
        </label>
      </div>
    </div>

    <div class="fixed left-0 right-0 bottom-0 bg-white/90 backdrop-blur border-t border-[#e5e7eb] py-3 px-6 shadow-[0_-6px_20px_rgba(52,64,84,0.08)] z-20">
      <div class="max-w-6xl mx-auto flex items-center justify-between gap-4">
        <div class="text-sm text-[#6b7280]">
          <span class="font-semibold text-[#111827]">{{ selectedTemplate?.name || '未选择模板' }}</span>
          <span class="ml-2">{{ selectedTemplate?.description || '请选择模板后开始转换' }}</span>
        </div>
        <div class="flex gap-3">
          <button class="bg-[#e5e7eb] text-[#374151] px-6 py-3 rounded-xl text-base font-bold cursor-pointer transition-all hover:bg-[#d1d5db]" @click="goBack">上一步</button>
          <button 
            v-if="selectedTemplate" 
            class="bg-[#6366f1] text-white px-4 py-2 rounded-lg text-base font-medium cursor-pointer transition-all hover:bg-[#4f46e5]" 
            @click="showConfigDialog"
          >
            配置选项
          </button>
          <button class="bg-[linear-gradient(90deg,#22c55e,#16a34a)] text-white px-7 py-3 rounded-xl text-base font-bold cursor-pointer transition-all shadow-[0_12px_30px_rgba(34,197,94,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(34,197,94,0.3)] disabled:opacity-60 disabled:cursor-not-allowed disabled:shadow-none" :disabled="!selectedTemplate || isLoading" @click="convertMarkdown">
            {{ isLoading ? '正在转换...' : '开始转换' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>