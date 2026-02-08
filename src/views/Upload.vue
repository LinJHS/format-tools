<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { pandocService, PreparedInput, PrepareInputPayload } from '../services/pandocService'
import DownloadProgress from '../components/DownloadProgress.vue'

const uploadStore = useUploadStore()
const router = useRouter()

const fileInput = ref<HTMLInputElement | null>(null)
const batchFileInput = ref<HTMLInputElement | null>(null)
const activeTab = ref<'file' | 'batch' | 'text'>('file')
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
const showMdSelectionDialog = ref(false)
const pendingMarkdownFiles = ref<string[]>([])
const selectedMdFile = ref<string>('')
const unlistenDragDrop = ref<(() => void) | null>(null)

const supportedExtensions = ['.md', '.markdown', '.txt', '.zip', '.7z', '.tar.gz', '.tar.xz']

const installDependencies = async () => {
  // ... (keep existing implementation)
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
    
    unlistenDragDrop.value = await window.onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        dragActive.value = true
      } else if (event.payload.type === 'drop') {
        dragActive.value = false
        const paths = event.payload.paths as string[]
        
        if (paths && paths.length > 0) {
          if (activeTab.value === 'batch') {
             // Handle multiple files
             const files: File[] = []
             let hasError = false
             for (const filePath of paths) {
                 const fileName = filePath.split(/[\\/]/).pop() || ''
                 if (isSupportedFile(fileName)) {
                     const fakeFile = createFakeFile(fileName, filePath)
                     files.push(fakeFile)
                 } else {
                     hasError = true
                 }
             }
             if (files.length > 0) {
                 uploadStore.addFiles(files)
                 prepareError.value = hasError ? '部分不支持的文件已被跳过' : ''
             } else {
                 prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
             }
          } else {
              // Handle single file (files[0])
              const filePath = paths[0]
              const fileName = filePath.split(/[\\/]/).pop() || ''
              
              if (isSupportedFile(fileName)) {
                const fakeFile = createFakeFile(fileName, filePath)
                selectedFile.value = fakeFile
                prepareError.value = ''
                activeTab.value = 'file'
              } else {
                prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
              }
          }
        }
      } else {
        dragActive.value = false
      }
    })
  } catch (error) {
    console.warn('无法设置拖拽监听器:', error)
  }
}

const createFakeFile = (name: string, path: string): File => {
    return {
      name: name,
      path: path, // Tauri custom property
      size: 0,
      type: '',
      lastModified: Date.now(),
      slice: () => new Blob(),
      stream: () => new ReadableStream(),
      text: () => Promise.resolve(''),
      arrayBuffer: () => Promise.resolve(new ArrayBuffer(0))
    } as any as File
}

onMounted(() => {
  installDependencies()
  setupFileDropListener()
  // Ensure store works with new tab
  if (uploadStore.mode === 'file' || uploadStore.mode === 'batch') {
      // maybe retain session
  }
})

onBeforeUnmount(() => {
  if (unlistenDragDrop.value) {
    unlistenDragDrop.value()
  }
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

const openBatchFilePicker = () => {
    batchFileInput.value?.click()
}

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = false

  const fileList = e.dataTransfer?.files
  if (fileList && fileList.length > 0) {
    if (activeTab.value === 'batch') {
        const files: File[] = []
        let hasError = false
        for (let i = 0; i < fileList.length; i++) {
            const file = fileList[i]
            if (isSupportedFile(file.name)) {
                files.push(file)
            } else {
                hasError = true
            }
        }
        if (files.length > 0) {
            uploadStore.addFiles(files)
            prepareError.value = hasError ? '部分不支持的文件已被跳过' : ''
        } else {
            prepareError.value = '仅支持指定格式的文件'
        }
    } else {
        // Single file
        const file = fileList[0]
        if (isSupportedFile(file.name)) {
          selectedFile.value = file
          prepareError.value = ''
        } else {
          prepareError.value = '仅支持 .md/.markdown/.txt/.zip/.7z/.tar.gz/.tar.xz 文件'
        }
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

const handleBatchFileSelect = (e: Event) => {
    const input = e.target as HTMLInputElement
    if (input.files && input.files.length > 0) {
        const files: File[] = []
        let hasError = false
        for (let i = 0; i < input.files.length; i++) {
            const file = input.files[i]
            if (isSupportedFile(file.name)) {
                files.push(file)
            } else {
                hasError = true
            }
        }
        if (files.length > 0) {
            uploadStore.addFiles(files)
            prepareError.value = hasError ? '部分不支持的文件已被跳过' : ''
        }
    }
    // clear input to allow re-selecting same files
    input.value = ''
}

const clearSelection = () => {
  selectedFile.value = null
  prepareError.value = ''
}

const removeBatchFile = (fileName: string) => {
    uploadStore.removeFile(fileName)
}

const switchTab = (tab: 'file' | 'batch' | 'text') => {
  activeTab.value = tab
  prepareError.value = '' // Clear error on switch
}

const confirmMdSelection = async () => {
    // Only triggered for single file zip selection in current logic
    // Logic remains same, updates store and pushes
  if (!selectedMdFile.value) {
    prepareError.value = '请选择一个 Markdown 文件'
    return
  }
  
  // Re-build payload manually since selectMdFile flow is specific
  if (!selectedFile.value) return 

  const filePath = (selectedFile.value as any).path as string | undefined
    if (!filePath) {
      prepareError.value = '无法获取文件路径'
      return
    }

  const payload: PrepareInputPayload = {
      source_type: 'file',
      path: filePath,
      original_name: selectedFile.value.name
  }

  try {
    isPreparing.value = true
    const prepared = await pandocService.prepareInput({
      ...payload,
      selected_markdown: selectedMdFile.value
    })

    uploadStore.setMode('file')
    uploadStore.addFiles([selectedFile.value])
    uploadStore.setPreparedInput(prepared) 
    uploadStore.setStep(2)
    showMdSelectionDialog.value = false
    router.push('/template')
  } catch (error) {
    console.error('预处理失败:', error)
    prepareError.value = '预处理失败'
  } finally {
    isPreparing.value = false
  }
}

const nextStep = async () => {
  if (uploadDisabled.value || isPreparing.value) return

  try {
    isPreparing.value = true
    const preparedResults: PreparedInput[] = []
    
    if (activeTab.value === 'text') {
        if (!textContent.value.trim()) {
            prepareError.value = '请输入 Markdown 内容'
            isPreparing.value = false
            return
        }
        // Text flow
        const payload: PrepareInputPayload = {
            source_type: 'text',
            content: textContent.value,
            suggested_name: 'input.md'
        }
        const prepared = await pandocService.prepareInput(payload)
        preparedResults.push(prepared)
        
        uploadStore.setMode('text')
        uploadStore.setMarkdownText(textContent.value)
        uploadStore.setPreparedInputs(preparedResults)
        
    } else if (activeTab.value === 'file') {
        if (!selectedFile.value) {
            prepareError.value = '请先选择要处理的文件'
            isPreparing.value = false
            return
        }
        const file = selectedFile.value
        const payload: PrepareInputPayload = {
             source_type: 'file', 
             path: (file as any).path, 
             original_name: file.name 
        }
        const prepared = await pandocService.prepareInput(payload)
        
        // Single file archive handling
        const fileName = file.name
        const isArchive = /\.(zip|7z|tar\.gz|tar\.xz)$/i.test(fileName)
        if (isArchive) {
             const mdFiles = prepared.markdown_files || []
             if (mdFiles.length > 1) {
                 pendingMarkdownFiles.value = mdFiles
                 selectedMdFile.value = mdFiles[0]
                 showMdSelectionDialog.value = true
                 isPreparing.value = false
                 return
             }
        }
        
        preparedResults.push(prepared)
        uploadStore.setMode('file')
        // Ensure store has the file (if added via single file tab)
        uploadStore.clearFiles()
        uploadStore.addFiles([file])
        uploadStore.setPreparedInputs(preparedResults)

    } else if (activeTab.value === 'batch') {
        if (uploadStore.files.length === 0) {
            prepareError.value = '请先添加要处理的文件'
            isPreparing.value = false
            return
        }
        
        // Process each file
        for (const file of uploadStore.files) {
             const payload: PrepareInputPayload = {
                 source_type: 'file',
                 path: (file as any).path,
                 original_name: file.name,
                 // For batch archives, we might default to first MD file or skip check?
                 // Since "share config" implies automated workflow, we'll auto-select first MD if multiple?
                 // Or we just let prepareInput pick the best/first one by default if not specified.
             }
             // Be careful: prepareInput needs to know which MD to pick if payload doesn't specify.
             // Backend `prepareInput` probably defaults to first or only MD.
             // If multiple, we might face issues.
             // For Batch, we assume auto-selection (first MD).
             const prepared = await pandocService.prepareInput(payload)
             preparedResults.push(prepared)
        }
        
        uploadStore.setMode('file') // Reuse 'file' mode for batch logic downstream
        uploadStore.setPreparedInputs(preparedResults)
    }

    uploadStore.setStep(2)
    router.push('/template')
    
  } catch (error) {
    console.error('预处理失败:', error)
    prepareError.value = `预处理失败: ${error instanceof Error ? error.message : String(error)}`
  } finally {
    isPreparing.value = false
  }
}
</script>

<template>
<div class="min-h-[calc(100vh-56px)] p-6 bg-[radial-gradient(circle_at_20%_20%,#f5f7ff,#eef2ff_40%,#e8edf8_80%)]">
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

    <div class="max-w-3xl mx-auto bg-white rounded-2xl p-6 shadow-[0_22px_80px_rgba(52,64,84,0.14)] hover:shadow-[0_26px_90px_rgba(52,64,84,0.18)]" :class="{ 'pointer-events-none opacity-40': uploadDisabled }">
      <div class="flex items-center justify-between gap-4 mb-5">
        <div>
          <h1 class="m-0 text-[#1f2937] text-2xl tracking-tight">开始转换</h1>
          <p class="m-0 mt-1 text-[#4b5563] text-sm">上传 Markdown 或直接粘贴内容。</p>
        </div>
        <span class="bg-[#e0e7ff] text-[#3730a3] px-3 py-2 rounded-xl font-semibold text-xs">Step 1 / 2</span>
      </div>

      <div class="inline-flex border border-[#e5e7eb] rounded-xl overflow-hidden mb-4 bg-[#f9fafb]">
        <button class="border-none px-4 py-2 font-semibold bg-transparent text-[#4b5563] cursor-pointer transition-all" :class="{ 'bg-[linear-gradient(90deg,#6366f1,#8b5cf6)] text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]': activeTab === 'file' }" @click="switchTab('file')">单文件上传</button>
        <button class="border-none px-4 py-2 font-semibold bg-transparent text-[#4b5563] cursor-pointer transition-all" :class="{ 'bg-[linear-gradient(90deg,#6366f1,#8b5cf6)] text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]': activeTab === 'batch' }" @click="switchTab('batch')">批量上传</button>
        <button class="border-none px-4 py-2 font-semibold bg-transparent text-[#4b5563] cursor-pointer transition-all" :class="{ 'bg-[linear-gradient(90deg,#6366f1,#8b5cf6)] text-white shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]': activeTab === 'text' }" @click="switchTab('text')">markdown 文本</button>
      </div>

      <!-- Single File Tab -->
      <div v-if="activeTab === 'file'" class="border border-dashed border-[#e5e7eb] rounded-xl p-4 bg-[#f8fafc] flex flex-col min-h-61">
        <div 
          class="border-2 border-dashed border-[#c7d2fe] rounded-xl p-6 text-center bg-white transition-all cursor-pointer flex-1 flex flex-col justify-center"
          :class="{ 'border-[#7c3aed] bg-[#f5f3ff] shadow-[0_10px_30px_rgba(124,58,237,0.16)]': dragActive }"
          @dragover="handleDragOver"
          @dragleave="handleDragLeave"
          @drop="handleDrop"
          @click="openFilePicker"
        >
          <input type="file" ref="fileInput" class="hidden" :accept="supportedExtensions.join(',')" @change="handleFileSelect" />
          <div class="text-3xl text-[#7c3aed] mb-1">⬆</div>
          <p class="m-0 font-bold text-[#111827] text-lg">点击或拖拽文件到这里</p>
          <p class="m-0 mt-1 text-[#4b5563] text-sm">支持 .md / .markdown / .txt / .zip / .7z / .tar.gz / .tar.xz</p>
          <p class="m-0 mt-1 text-[#6b7280] text-xs">一切数据将全部在本地处理，确保您的隐私。</p>
        </div>

        <div v-if="selectedFile" class="mt-1 flex flex-1 items-center justify-between gap-2 px-3 py-2 rounded-lg bg-[#eef2ff] text-[#1f2937]">
          <span class="truncate text-m font-semibold">{{ selectedFile.name }}</span>
          <button class="border-none bg-[#e0e7ff] text-[#312e81] rounded-full w-5 h-5 cursor-pointer font-bold text-xs flex items-center justify-center shrink-0" @click.stop="clearSelection">✕</button>
        </div>
      </div>

      <!-- Batch Upload Tab -->
      <div v-else-if="activeTab === 'batch'" class="border border-dashed border-[#e5e7eb] rounded-xl p-4 bg-[#f8fafc] flex flex-col min-h-61">
        <div 
          class="border-2 border-dashed border-[#c7d2fe] rounded-xl p-6 text-center bg-white transition-all cursor-pointer flex-none flex flex-col justify-center mb-4"
          :class="{ 'border-[#7c3aed] bg-[#f5f3ff] shadow-[0_10px_30px_rgba(124,58,237,0.16)]': dragActive }"
          @dragover="handleDragOver"
          @dragleave="handleDragLeave"
          @drop="handleDrop"
          @click="openBatchFilePicker"
        >
          <input type="file" ref="batchFileInput" class="hidden" multiple :accept="supportedExtensions.join(',')" @change="handleBatchFileSelect" />
          <div class="text-3xl text-[#7c3aed] mb-1">📚</div>
          <p class="m-0 font-bold text-[#111827] text-lg">点击或拖拽多个文件到这里</p>
          <p class="m-0 mt-1 text-[#4b5563] text-sm">支持 .md / .zip / .7z 等格式批量转换</p>
        </div>

        <!-- File List -->
        <div class="flex-1 overflow-y-auto max-h-48 space-y-2 pr-1">
             <div v-if="uploadStore.files.length === 0" class="text-center text-gray-400 py-4 text-sm">
                 暂未添加文件
             </div>
             <div v-for="file in uploadStore.files" :key="file.name" class="flex items-center justify-between gap-2 px-3 py-2 rounded-lg bg-white border border-gray-100 shadow-sm">
                  <span class="truncate text-sm font-medium text-gray-700">{{ file.name }}</span>
                  <button class="border-none bg-red-50 text-red-500 hover:bg-red-100 rounded-full w-5 h-5 cursor-pointer font-bold text-xs flex items-center justify-center shrink-0 transition-colors" @click.stop="removeBatchFile(file.name)">✕</button>
             </div>
        </div>
      </div>

      <!-- Text Tab -->
      <div v-else class="border border-dashed border-[#e5e7eb] rounded-xl px-4 py-3 bg-[#f8fafc]">
        <label class="block mb-2 font-bold text-[#1f2937]">Markdown 文本</label>
        <textarea v-model="textContent" class="w-full border border-[#e5e7eb] rounded-xl p-3.5 text-sm font-mono bg-white text-[#111827] resize-y min-h-45" placeholder="粘贴你的 Markdown，图片引用会被自动扫描并复制到临时目录。" rows="7"></textarea>
      </div>

      <div v-if="prepareError" class="mt-3 px-3.5 py-3 rounded-xl bg-[#fef2f2] text-[#b91c1c] border border-[#fecdd3]">{{ prepareError }}</div>

      <div class="flex justify-end mt-5">
        <button class="bg-[linear-gradient(90deg,#22c55e,#16a34a)] text-white px-7 py-3 rounded-xl text-base font-bold cursor-pointer transition-all shadow-[0_12px_30px_rgba(34,197,94,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(34,197,94,0.3)] disabled:opacity-60 disabled:cursor-not-allowed disabled:shadow-none" :disabled="uploadDisabled || isPreparing" @click="nextStep">
          {{ isPreparing ? '正在整理...' : '下一步' }}
        </button>
      </div>
    </div>
  </div>

  <!-- MD 文件选择对话框 -->
  <div v-if="showMdSelectionDialog" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-2xl p-6 shadow-[0_25px_50px_rgba(0,0,0,0.25)] max-w-sm w-11/12">
      <h2 class="m-0 text-[#1f2937] text-xl font-bold mb-4">选择 Markdown 文件</h2>
      <p class="m-0 text-[#6b7280] text-sm mb-4">压缩包中找到 {{ pendingMarkdownFiles.length }} 个 Markdown 文件，请选择要转换的文件：</p>
      
      <div class="mb-6 border border-[#e5e7eb] rounded-xl overflow-hidden bg-[#f9fafb] max-h-64 overflow-y-auto">
        <div v-for="(mdFile, idx) in pendingMarkdownFiles" :key="idx" class="p-3 border-b border-[#e5e7eb] last:border-b-0 cursor-pointer hover:bg-[#f3f4f6] transition-all" :class="{ 'bg-[#eef2ff] border-l-4 border-l-[#6366f1]': selectedMdFile === mdFile }" @click="selectedMdFile = mdFile">
          <div class="flex items-center gap-2">
            <input type="radio" :checked="selectedMdFile === mdFile" class="cursor-pointer" />
            <span class="text-sm text-[#111827] font-medium truncate">{{ mdFile.split(/[\\/]/).pop() }}</span>
          </div>
        </div>
      </div>

      <div class="flex gap-3 justify-end">
        <button class="px-5 py-2 rounded-lg text-sm font-semibold text-[#6b7280] bg-[#f3f4f6] cursor-pointer transition-all hover:bg-[#e5e7eb]" @click="showMdSelectionDialog = false">取消</button>
        <button class="px-5 py-2 rounded-lg text-sm font-semibold text-white bg-[linear-gradient(90deg,#22c55e,#16a34a)] cursor-pointer transition-all shadow-[0_12px_30px_rgba(34,197,94,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(34,197,94,0.3)]" @click="confirmMdSelection">确认</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 样式主要由 Tailwind 提供；这里保留空块用于后续微调 */
</style>
