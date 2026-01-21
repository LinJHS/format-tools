<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { pandocService, PreparedInput, PrepareInputPayload } from '../services/pandocService'
import DownloadProgress from '../components/DownloadProgress.vue'

const uploadStore = useUploadStore()
const router = useRouter()

const fileInput = ref<HTMLInputElement | null>(null)
const activeTab = ref<'file' | 'text'>('file')
const selectedFile = ref<File | null>(null)
const textContent = ref('')
const dragActive = ref(false)
const isInstalling = ref(false)
const installTitle = ref('正在准备必要组件')
const installDetail = ref('')
const downloadProgress = ref({ downloaded: 0, total: 0, percentage: 0 })
const installError = ref('')
const isError = ref(false)
const uploadDisabled = ref(false)
const isPreparing = ref(false)
const prepareError = ref('')

const supportedExtensions = ['.md', '.markdown', '.txt', '.zip', '.7z', '.tar.gz', '.tar.xz']

const installDependencies = async () => {
  try {
    isError.value = false
    installError.value = ''
    uploadDisabled.value = false

    const pandocInstalled = await pandocService.isPandocInstalled()
    if (!pandocInstalled) {
      isInstalling.value = true
      installTitle.value = '正在准备必要组件'
      installDetail.value = '正在下载 Pandoc'
      await pandocService.installPandoc((progress) => {
        downloadProgress.value = progress
      })
    }

    const crossrefInstalled = await pandocService.isCrossrefInstalled()
    if (!crossrefInstalled) {
      isInstalling.value = true
      installTitle.value = '正在准备必要组件'
      installDetail.value = '正在下载 Pandoc-crossref'
      downloadProgress.value = { downloaded: 0, total: 0, percentage: 0 }
      await pandocService.installCrossref((progress) => {
        downloadProgress.value = progress
      })
    }

    isInstalling.value = false
    installDetail.value = ''
  } catch (error) {
    console.error('安装失败:', error)
    isInstalling.value = true
    isError.value = true
    installTitle.value = '组件下载失败'
    installError.value = installDetail.value || '组件下载失败，请检查网络连接后重试。'
    uploadDisabled.value = true
  }
}

const handleRetry = () => {
  installDependencies()
}

const handleGoHome = () => {
  router.push('/')
}

onMounted(() => {
  installDependencies()
})

const isSupportedFile = (fileName: string) => {
  const lower = fileName.toLowerCase()
  return supportedExtensions.some((ext) => lower.endsWith(ext))
}

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = true
}

const handleDragLeave = () => {
  dragActive.value = false
}

const openFilePicker = () => {
  fileInput.value?.click()
}

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = false

  const fileList = e.dataTransfer?.files
  if (fileList && fileList.length > 0) {
    const file = fileList[0]
    if (isSupportedFile(file.name)) {
      selectedFile.value = file
      prepareError.value = ''
    } else {
      prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
    }
  }
}

const handleFileSelect = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files && input.files.length > 0) {
    const file = input.files[0]
    if (isSupportedFile(file.name)) {
      selectedFile.value = file
      prepareError.value = ''
    } else {
      prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
    }
  }
}

const clearSelection = () => {
  selectedFile.value = null
  prepareError.value = ''
}

const switchTab = (tab: 'file' | 'text') => {
  activeTab.value = tab
  prepareError.value = ''
}

const buildPayload = (): PrepareInputPayload | null => {
  if (activeTab.value === 'file') {
    if (!selectedFile.value) {
      prepareError.value = '请先选择要处理的文件'
      return null
    }

    const filePath = (selectedFile.value as any).path as string | undefined
    if (!filePath) {
      prepareError.value = '无法获取文件路径，请通过桌面客户端选择文件'
      return null
    }

    return {
      source_type: 'file',
      path: filePath,
      original_name: selectedFile.value.name
    }
  }

  if (!textContent.value.trim()) {
    prepareError.value = '请输入 Markdown 内容'
    return null
  }

  return {
    source_type: 'text',
    content: textContent.value,
    suggested_name: 'input.md'
  }
}

const nextStep = async () => {
  if (uploadDisabled.value || isPreparing.value) {
    return
  }

  const payload = buildPayload()
  if (!payload) return

  try {
    isPreparing.value = true
    const prepared: PreparedInput = await pandocService.prepareInput(payload)

    uploadStore.setMode(activeTab.value)
    uploadStore.addFiles(selectedFile.value ? [selectedFile.value] : [])
    uploadStore.setMarkdownText(textContent.value)
    uploadStore.setPreparedInput(prepared)
    uploadStore.setStep(2)
    router.push('/template')
  } catch (error) {
    console.error('预处理失败:', error)
    prepareError.value = '预处理失败，请检查文件内容或稍后重试'
  } finally {
    isPreparing.value = false
  }
}
</script>

<template>
  <div class="upload-container">
    <!-- 下载进度弹窗 -->
    <DownloadProgress 
      :is-visible="isInstalling"
      :title="installTitle"
      :detail="installDetail"
      :progress="downloadProgress"
      :is-error="isError"
      :error-message="installError"
      @retry="handleRetry"
      @go-home="handleGoHome"
    />
    
    <div class="upload-content" :class="{ 'pointer-events-none opacity-40': uploadDisabled }">
      <div class="header">
        <div>
          <h1>开始转换</h1>
          <p class="subtitle">上传 Markdown 或直接粘贴内容，我们会自动提取图片并放到临时目录。</p>
        </div>
        <span class="badge">Step 1 / 2</span>
      </div>

      <div class="tabs">
        <button
          class="tab"
          :class="{ active: activeTab === 'file' }"
          @click="switchTab('file')"
        >
          上传文件
        </button>
        <button
          class="tab"
          :class="{ active: activeTab === 'text' }"
          @click="switchTab('text')"
        >
          输入 Markdown 文本
        </button>
      </div>

      <div v-if="activeTab === 'file'" class="panel">
        <div 
          class="drop-area"
          :class="{ active: dragActive }"
          @dragover="handleDragOver"
          @dragleave="handleDragLeave"
          @drop="handleDrop"
          @click="openFilePicker"
        >
          <input
            type="file"
            ref="fileInput"
            class="hidden"
            :accept="supportedExtensions.join(',')"
            @change="handleFileSelect"
          />
          <div class="drop-icon">⬆</div>
          <p class="drop-title">点击或拖拽文件到这里</p>
          <p class="drop-desc">支持 .md / .markdown / .txt / .zip / .7z / .tar.gz / .tar.xz</p>
          <p class="drop-note">我们会自动扫描 Markdown 并复制引用的图片到临时文件夹。</p>
        </div>

        <div v-if="selectedFile" class="selected">
          <div class="file-chip">
            <span class="file-name">{{ selectedFile.name }}</span>
            <button class="chip-close" @click.stop="clearSelection">✕</button>
          </div>
        </div>
      </div>

      <div v-else class="panel">
        <label class="field-label">Markdown 文本</label>
        <textarea
          v-model="textContent"
          class="text-input"
          placeholder="粘贴你的 Markdown，图片引用会被自动扫描并复制到临时目录。"
          rows="12"
        ></textarea>
      </div>

      <div v-if="prepareError" class="error-box">{{ prepareError }}</div>

      <div class="button-group">
        <button class="btn-next" :disabled="uploadDisabled || isPreparing" @click="nextStep">
          {{ isPreparing ? '正在整理...' : '下一步' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.upload-container {
  min-height: 100vh;
  padding: 32px;
  background: radial-gradient(circle at 20% 20%, #f5f7ff, #eef2ff 40%, #e8edf8 80%);
}

.upload-content {
  max-width: 960px;
  margin: 0 auto;
  background: #ffffff;
  border-radius: 16px;
  padding: 32px;
  box-shadow: 0 22px 80px rgba(52, 64, 84, 0.14);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.upload-content:hover {
  transform: translateY(-2px);
  box-shadow: 0 26px 90px rgba(52, 64, 84, 0.18);
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 20px;
}

h1 {
  margin: 0;
  color: #1f2937;
  font-size: 28px;
  letter-spacing: -0.4px;
}

.subtitle {
  margin: 4px 0 0 0;
  color: #4b5563;
  font-size: 14px;
}

.badge {
  background: #e0e7ff;
  color: #3730a3;
  padding: 8px 12px;
  border-radius: 12px;
  font-weight: 600;
  font-size: 12px;
}

.tabs {
  display: inline-flex;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  overflow: hidden;
  margin-bottom: 16px;
  background: #f9fafb;
}

.tab {
  border: none;
  padding: 12px 18px;
  font-weight: 600;
  background: transparent;
  color: #4b5563;
  cursor: pointer;
  transition: all 0.2s ease;
}

.tab.active {
  background: linear-gradient(90deg, #6366f1, #8b5cf6);
  color: #fff;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);
}

.tab:not(.active):hover {
  background: #eef2ff;
}

.panel {
  border: 1px dashed #e5e7eb;
  border-radius: 14px;
  padding: 20px;
  background: #f8fafc;
}

.drop-area {
  border: 2px dashed #c7d2fe;
  border-radius: 14px;
  padding: 32px;
  text-align: center;
  background: #fff;
  transition: all 0.25s ease;
  cursor: pointer;
}

.drop-area.active {
  border-color: #7c3aed;
  background: #f5f3ff;
  box-shadow: 0 10px 30px rgba(124, 58, 237, 0.16);
}

.drop-icon {
  font-size: 32px;
  color: #7c3aed;
  margin-bottom: 10px;
}

.drop-title {
  margin: 0;
  font-weight: 700;
  color: #111827;
  font-size: 18px;
}

.drop-desc {
  margin: 6px 0;
  color: #4b5563;
  font-size: 14px;
}

.drop-note {
  margin: 0;
  color: #6b7280;
  font-size: 12px;
}

.selected {
  margin-top: 12px;
}

.file-chip {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-radius: 12px;
  background: #eef2ff;
  color: #1f2937;
  font-weight: 600;
}

.file-name {
  max-width: 420px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chip-close {
  border: none;
  background: #e0e7ff;
  color: #312e81;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  cursor: pointer;
  font-weight: 700;
}

.field-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 700;
  color: #1f2937;
}

.text-input {
  width: 100%;
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  padding: 14px;
  font-size: 14px;
  font-family: "JetBrains Mono", "SFMono-Regular", Consolas, monospace;
  background: #fff;
  color: #111827;
  resize: vertical;
  min-height: 260px;
}

.error-box {
  margin-top: 14px;
  padding: 12px 14px;
  border-radius: 12px;
  background: #fef2f2;
  color: #b91c1c;
  border: 1px solid #fecdd3;
}

.button-group {
  display: flex;
  justify-content: flex-end;
  margin-top: 20px;
}

.btn-next {
  background: linear-gradient(90deg, #22c55e, #16a34a);
  color: #fff;
  padding: 14px 28px;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  box-shadow: 0 12px 30px rgba(34, 197, 94, 0.25);
}

.btn-next:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  box-shadow: none;
}

.btn-next:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 14px 34px rgba(34, 197, 94, 0.3);
}

@media (max-width: 768px) {
  .upload-container {
    padding: 18px;
  }

  .upload-content {
    padding: 20px;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
  }

  h1 {
    font-size: 22px;
  }
}
</style>
