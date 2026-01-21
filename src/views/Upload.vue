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

// 监听Tauri拖拽事件
const setupFileDropListener = async () => {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const window = getCurrentWindow()
    
    const unlisten = await window.onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        // 用户正在拖拽悬停
        dragActive.value = true
      } else if (event.payload.type === 'drop') {
        // 用户放下文件
        dragActive.value = false
        const paths = event.payload.paths as string[]
        
        if (paths && paths.length > 0) {
          const filePath = paths[0]
          const fileName = filePath.split('\\').pop() || filePath.split('/').pop() || ''
          
          if (isSupportedFile(fileName)) {
            // 创建一个虚拟File对象，但添加path属性用于Tauri
            const fakeFile = {
              name: fileName,
              path: filePath,
              size: 0,
              type: '',
              lastModified: Date.now(),
              slice: () => new Blob(),
              stream: () => new ReadableStream(),
              text: () => Promise.resolve(''),
              arrayBuffer: () => Promise.resolve(new ArrayBuffer(0))
            } as any as File
            
            selectedFile.value = fakeFile
            prepareError.value = ''
            activeTab.value = 'file'
          } else {
            prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
          }
        }
      } else {
        // 文件拖拽取消
        dragActive.value = false
      }
    })
    
    // 注意: 如果组件卸载需要调用 unlisten()
    // 这里暂不处理，因为这是主要页面
  } catch (error) {
    console.warn('无法设置拖拽监听器:', error)
  }
}

onMounted(() => {
  installDependencies()
  setupFileDropListener()
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
  <div class="min-h-screen p-8 bg-[radial-gradient(circle_at_20%_20%,_#f5f7ff,_#eef2ff_40%,_#e8edf8_80%)]">
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

    <div class="max-w-3xl mx-auto bg-white rounded-2xl p-8 shadow-[0_22px_80px_rgba(52,64,84,0.14)] transition-all hover:-translate-y-0.5 hover:shadow-[0_26px_90px_rgba(52,64,84,0.18)]" :class="{ 'pointer-events-none opacity-40': uploadDisabled }">
      <div class="flex items-center justify-between gap-4 mb-5">
        <div>
          <h1 class="m-0 text-[#1f2937] text-2xl tracking-tight">开始转换</h1>
          <p class="m-0 mt-1 text-[#4b5563] text-sm">上传 Markdown 或直接粘贴内容，我们会自动提取图片并放到临时目录。</p>
        </div>
        <span class="bg-[#e0e7ff] text-[#3730a3] px-3 py-2 rounded-xl font-semibold text-xs">Step 1 / 2</span>
      </div>

      <div class="inline-flex border border-[#e5e7eb] rounded-xl overflow-hidden mb-4 bg-[#f9fafb]">
        <button class="border-none px-4 py-3 font-semibold bg-transparent text-[#4b5563] cursor-pointer transition-all" :class="{ 'bg-[linear-gradient(90deg,_#6366f1,_#8b5cf6)] text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]': activeTab === 'file' }" @click="switchTab('file')">上传文件</button>
        <button class="border-none px-4 py-3 font-semibold bg-transparent text-[#4b5563] cursor-pointer transition-all" :class="{ 'bg-[linear-gradient(90deg,_#6366f1,_#8b5cf6)] text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]': activeTab === 'text' }" @click="switchTab('text')">输入 Markdown 文本</button>
      </div>

      <div v-if="activeTab === 'file'" class="border border-dashed border-[#e5e7eb] rounded-xl p-5 bg-[#f8fafc]">
        <div 
          class="border-2 border-dashed border-[#c7d2fe] rounded-xl p-8 text-center bg-white transition-all cursor-pointer"
          :class="{ 'border-[#7c3aed] bg-[#f5f3ff] shadow-[0_10px_30px_rgba(124,58,237,0.16)]': dragActive }"
          @dragover="handleDragOver"
          @dragleave="handleDragLeave"
          @drop="handleDrop"
          @click="openFilePicker"
        >
          <input type="file" ref="fileInput" class="hidden" :accept="supportedExtensions.join(',')" @change="handleFileSelect" />
          <div class="text-3xl text-[#7c3aed] mb-2">⬆</div>
          <p class="m-0 font-bold text-[#111827] text-lg">点击或拖拽文件到这里</p>
          <p class="m-0 mt-1 text-[#4b5563] text-sm">支持 .md / .markdown / .txt / .zip / .7z / .tar.gz / .tar.xz</p>
          <p class="m-0 mt-1 text-[#6b7280] text-xs">我们会自动扫描 Markdown 并复制引用的图片到临时文件夹。</p>
        </div>

        <div v-if="selectedFile" class="mt-3">
          <div class="inline-flex items-center gap-2.5 px-3.5 py-2.5 rounded-xl bg-[#eef2ff] text-[#1f2937] font-bold">
            <span class="max-w-[420px] overflow-hidden text-ellipsis whitespace-nowrap">{{ selectedFile.name }}</span>
            <button class="border-none bg-[#e0e7ff] text-[#312e81] rounded-full w-6 h-6 cursor-pointer font-bold" @click.stop="clearSelection">✕</button>
          </div>
        </div>
      </div>

      <div v-else class="border border-dashed border-[#e5e7eb] rounded-xl p-5 bg-[#f8fafc]">
        <label class="block mb-2 font-bold text-[#1f2937]">Markdown 文本</label>
        <textarea v-model="textContent" class="w-full border border-[#e5e7eb] rounded-xl p-3.5 text-sm font-mono bg-white text-[#111827] resize-y min-h-[260px]" placeholder="粘贴你的 Markdown，图片引用会被自动扫描并复制到临时目录。" rows="12"></textarea>
      </div>

      <div v-if="prepareError" class="mt-3 px-3.5 py-3 rounded-xl bg-[#fef2f2] text-[#b91c1c] border border-[#fecdd3]">{{ prepareError }}</div>

      <div class="flex justify-end mt-5">
        <button class="bg-[linear-gradient(90deg,_#22c55e,_#16a34a)] text-white px-7 py-3 rounded-xl text-base font-bold cursor-pointer transition-all shadow-[0_12px_30px_rgba(34,197,94,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(34,197,94,0.3)] disabled:opacity-60 disabled:cursor-not-allowed disabled:shadow-none" :disabled="uploadDisabled || isPreparing" @click="nextStep">
          {{ isPreparing ? '正在整理...' : '下一步' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 样式主要由 Tailwind 提供；这里保留空块用于后续微调 */
</style>
