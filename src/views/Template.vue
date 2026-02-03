<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { openUrl } from '@tauri-apps/plugin-opener'
import { LINKS } from '../config/links'
import { pandocService, TemplateInfo, TemplateMeta, ConvertOptions } from '../services/pandocService'
import { buildPandocMetadata, mergeConfigs } from '../services/configTransform'
import { saveRecentConfig } from '../services/configStorage'
import { DEFAULT_CONFIG } from '../types/templateConfig'
import type { TemplateConfig, ConfigPreset } from '../types/templateConfig'
import TemplateConfigDialog from '../components/TemplateConfigDialog.vue'
import PresetDialog from '../components/PresetDialog.vue'

const uploadStore = useUploadStore()
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
  
  // Initialize auth store if enabled
  if (authEnabled) {
    try {
      const { useAuthStore } = await import('../auth-private/stores/auth')
      authStore = useAuthStore()
      
      // Check if user has active ultra membership
      const membership = authStore.activeMembership
      isUltraMember.value = membership?.membershipType === 'ultra'
    } catch (e) {
      console.error('Error loading auth store:', e)
    }
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
    loadingMessage.value = '转换中...'

    // Check if AI format fix is needed
    if (useAIFix.value && authEnabled && isUltraMember.value && authStore) {
      try {
        loadingMessage.value = 'AI 格式修复中...'
        console.log('Starting AI format fixing...')
        
        // Import AI service
        const { fixMarkdownFormat } = await import('../auth-private/services/aiFormatService')
        const { readTextFile, writeTextFile } = await import('@tauri-apps/plugin-fs')
        
        // Get prepared input
        const preparedInput = uploadStore.preparedInput
        if (!preparedInput) {
          throw new Error('Input not prepared')
        }
        
        // Read markdown content
        const markdownContent = await readTextFile(preparedInput.markdown_path)
        
        // Fix markdown format with AI
        const fixedContent = await fixMarkdownFormat({
          ultraToken: authStore.ultraToken!,
          markdownContent
        })
        
        // Write fixed content back
        await writeTextFile(preparedInput.markdown_path, fixedContent)
        console.log('AI format fixing completed')
      } catch (aiError) {
        console.error('AI format fixing failed:', aiError)
        error.value = `AI 格式修复失败: ${aiError instanceof Error ? aiError.message : String(aiError)}`
        isLoading.value = false
        return
      }
    }

    loadingMessage.value = '模板准备中...'

    // 合并配置（用户配置 > 模板预设 > 默认配置）
    const finalConfig = mergeConfigs(
      userConfig.value,
      (selectedTemplate.value.defaultPreset as Partial<TemplateConfig>) || {},
      DEFAULT_CONFIG
    )
    
    // 转换为 Pandoc 元数据
    const pandocMetadata = buildPandocMetadata(finalConfig)
    
    // 保存到历史记录
    saveRecentConfig(finalConfig)

    // Prepare template
    const templateInfo: TemplateInfo = await pandocService.prepareTemplate(
      selectedTemplate.value.id,
      selectedTemplate.value.member
    )

    loadingMessage.value = '文档转换中...'

    // Prepare convert options
    const preparedInput = uploadStore.preparedInput
    if (!preparedInput) {
      throw new Error('Input not prepared')
    }

    const convertOptions: ConvertOptions = {
      input_file: preparedInput.markdown_path,
      source_dir: preparedInput.source_dir,
      source_name: preparedInput.source_name,
      reference_doc: templateInfo.reference_doc,
      metadata: pandocMetadata,  // 传递元数据
      metadata_file: undefined,
      use_crossref: true
    }

    // Convert markdown
    const outputPath = await pandocService.convertMarkdown(convertOptions)

    // Show success message and navigate
    console.log('转换完成:', outputPath)
    uploadStore.setOutputPath(outputPath)
    uploadStore.setStep(3)
    router.push('/result')
  } catch (err) {
    console.error('转换失败:', err)
    error.value = `转换失败: ${err instanceof Error ? err.message : String(err)}`
  } finally {
    isLoading.value = false
    loadingMessage.value = '转换中...'
  }
}

const goBack = () => {
  router.push('/upload')
}

const freeTemplates = computed(() => {
  return templates.value.filter((t) => !t.member)
})

const memberTemplates = computed(() => {
  if (!authEnabled) return []
  return templates.value.filter((t) => t.member)
})

// 配置对话框事件处理
const handleConfigConfirm = (config: Partial<TemplateConfig>) => {
  userConfig.value = config
}

const handleConfigDialogLoadPreset = () => {
  configDialogVisible.value = false
  presetDialogMode.value = 'load'
  presetDialogVisible.value = true
}

const handleConfigDialogSavePreset = () => {
  configDialogVisible.value = false
  presetDialogMode.value = 'save'
  presetDialogVisible.value = true
}

const handleConfigDialogReset = () => {
  initConfig()
}

const showConfigDialog = () => {
  configDialogVisible.value = true
}

// 预设对话框事件处理
const handlePresetLoad = (config: Partial<TemplateConfig>) => {
  userConfig.value = { ...config }
  configDialogVisible.value = true // 加载预设后重新打开配置对话框
}

const handlePresetSave = (preset: ConfigPreset) => {
  console.log('预设已保存:', preset.name)
  configDialogVisible.value = true // 保存预设后重新打开配置对话框
}

const handlePresetDialogClose = () => {
  presetDialogVisible.value = false
}

const openDownloadPage = async () => {
  await openUrl(LINKS.releases)
}

const isSelected = (template: TemplateMeta) => {
  return selectedTemplate.value?.id === template.id
}
</script>

<template>
  <div class="p-6 pb-28 bg-[radial-gradient(circle_at_20%_20%,#f5f7ff,#eef2ff_40%,#e8edf8_80%)]">
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

    <div class="fixed left-0 right-0 bottom-0 bg-white/90 backdrop-blur border-t border-[#e5e7eb] py-3 px-6 shadow-[0_-6px_20px_rgba(52,64,84,0.08)]">
      <div class="max-w-6xl mx-auto flex items-center justify-between gap-4">
        <div class="flex items-center gap-4">
          <div class="text-sm text-[#6b7280]">
            <span class="font-semibold text-[#111827]">{{ selectedTemplate?.name || '未选择模板' }}</span>
            <span class="ml-2">{{ selectedTemplate?.description || '请选择模板后开始转换' }}</span>
          </div>
          <label 
            v-if="isUltraMember" 
            class="flex items-center gap-2 cursor-pointer bg-gradient-to-r from-[#fef3c7] to-[#fde68a] text-[#92400e] px-3 py-1.5 rounded-lg text-xs font-semibold border-2 border-[#fbbf24] hover:from-[#fde68a] hover:to-[#fcd34d] transition-all"
          >
            <input 
              v-model="useAIFix" 
              type="checkbox" 
              class="w-4 h-4 accent-[#f59e0b] cursor-pointer"
            />
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
              <path d="M2 17l10 5 10-5"></path>
              <path d="M2 12l10 5 10-5"></path>
            </svg>
            <span>AI 格式修复</span>
          </label>
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
            {{ isLoading ? loadingMessage : '开始转换' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>