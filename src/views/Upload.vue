<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { pandocService } from '../services/pandocService'
import DownloadProgress from '../components/DownloadProgress.vue'

const uploadStore = useUploadStore()
const router = useRouter()

const files = ref<File[]>([])
const dragActive = ref(false)
const uploadMethod = ref('text') // text, file, folder
const isInstalling = ref(false)
const installTitle = ref('正在准备必要组件')
const installDetail = ref('')
const downloadProgress = ref({ downloaded: 0, total: 0, percentage: 0 })
const installError = ref('')
const isError = ref(false)
const uploadDisabled = ref(false) // 禁用上传功能

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

// 重试下载
const handleRetry = () => {
  installDependencies()
}

// 返回首页
const handleGoHome = () => {
  router.push('/')
}

// 检查并安装 Pandoc
onMounted(() => {
  installDependencies()
})

const handleDragOver = (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = true
}

const handleDragLeave = () => {
  dragActive.value = false
}

const handleDrop = (e: DragEvent) => {
  e.preventDefault()
  dragActive.value = false
  
  if (e.dataTransfer?.files) {
    files.value = Array.from(e.dataTransfer.files).filter(file =>
      file.name.endsWith('.md') || file.name.endsWith('.txt')
    )
  }
}

const handleFileSelect = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files) {
    files.value = Array.from(input.files).filter(file =>
      file.name.endsWith('.md') || file.name.endsWith('.txt')
    )
  }
}

const handleFolderSelect = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files) {
    files.value = Array.from(input.files).filter(file =>
      file.name.endsWith('.md') || file.name.endsWith('.txt')
    )
  }
}

const handleCompressedFileSelect = (e: Event) => {
  const input = e.target as HTMLInputElement
  if (input.files) {
    files.value = Array.from(input.files).filter(file => {
      const ext = file.name.toLowerCase()
      return ext.endsWith('.zip') || ext.endsWith('.rar') || ext.endsWith('.7z')
    })
  }
}

const nextStep = () => {
  if (uploadDisabled.value) {
    return
  }
  if (files.value.length > 0 || uploadMethod.value === 'text') {
    uploadStore.addFiles(files.value)
    uploadStore.setStep(2)
    router.push('/template')
  }
}

const clearSelection = () => {
  files.value = []
  uploadMethod.value = 'text'
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
    
    <div class="upload-content" :class="{ 'pointer-events-none opacity-50': uploadDisabled }">
      <h1>开始转换</h1>
      
      <div class="upload-methods">
        <!-- 文本框 -->
        <div class="method-section">
          <h3>直接输入文本</h3>
          <textarea 
            placeholder="将您的内容粘贴到这里..." 
            class="text-input"
          ></textarea>
        </div>

        <!-- 文件上传 -->
        <div class="method-section">
          <h3>上传文件 (.txt / .md)</h3>
          <div class="file-upload-area">
            <input 
              type="file" 
              ref="fileInput" 
              accept=".txt,.md"
              @change="handleFileSelect"
              hidden
            />
            <button class="btn-upload" @click="() => $refs.fileInput?.click()">
              选择文件
            </button>
          </div>
        </div>

        <!-- 文件夹选择 -->
        <div class="method-section">
          <h3>选择文件夹</h3>
          <div class="folder-upload-area">
            <input 
              type="file" 
              ref="folderInput" 
              webkitdirectory
              @change="handleFolderSelect"
              hidden
            />
            <button class="btn-upload" @click="() => $refs.folderInput?.click()">
              选择文件夹
            </button>
          </div>
        </div>

        <!-- 压缩文件 -->
        <div class="method-section">
          <h3>上传压缩文件 (.zip / .rar / .7z)</h3>
          <div class="compress-upload-area">
            <input 
              type="file" 
              ref="compressInput" 
              accept=".zip,.rar,.7z"
              @change="handleCompressedFileSelect"
              hidden
            />
            <button class="btn-upload" @click="() => $refs.compressInput?.click()">
              选择压缩文件
            </button>
          </div>
        </div>

        <!-- 拖拽区域 -->
        <div 
          class="drag-drop-area"
          :class="{ active: dragActive }"
          @dragover="handleDragOver"
          @dragleave="handleDragLeave"
          @drop="handleDrop"
        >
          <p>或将文件拖放到这里</p>
          <small>支持 .txt, .md, .zip, .rar, .7z 格式</small>
        </div>
      </div>

      <!-- 已选择文件列表 -->
      <div v-if="files.length > 0" class="selected-files">
        <h3>已选择文件</h3>
        <ul>
          <li v-for="(file, index) in files" :key="index">
            {{ file.name }}
          </li>
        </ul>
        <button class="btn-clear" @click="clearSelection">清除</button>
      </div>

      <!-- 底部按钮 -->
      <div class="button-group">
        <button class="btn-next" @click="nextStep">下一步</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.upload-container {
  min-height: 100vh;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.upload-content {
  max-width: 800px;
  margin: 0 auto;
  background: white;
  border-radius: 12px;
  padding: 40px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
}

h1 {
  text-align: center;
  color: #333;
  margin-bottom: 30px;
  font-size: 28px;
}

.upload-methods {
  margin-bottom: 30px;
}

.method-section {
  margin-bottom: 20px;
}

.method-section h3 {
  color: #555;
  font-size: 16px;
  margin-bottom: 10px;
}

.text-input {
  width: 100%;
  height: 120px;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-family: Arial, sans-serif;
  font-size: 14px;
  resize: vertical;
}

.file-upload-area,
.folder-upload-area,
.compress-upload-area {
  margin-bottom: 15px;
}

.btn-upload {
  background-color: #667eea;
  color: white;
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.3s;
}

.btn-upload:hover {
  background-color: #5568d3;
}

.drag-drop-area {
  border: 2px dashed #ddd;
  border-radius: 8px;
  padding: 40px;
  text-align: center;
  transition: all 0.3s;
  cursor: pointer;
  background-color: #f9f9f9;
}

.drag-drop-area.active {
  border-color: #667eea;
  background-color: #f0f4ff;
}

.drag-drop-area p {
  color: #666;
  margin: 0 0 5px 0;
}

.drag-drop-area small {
  color: #999;
}

.selected-files {
  background-color: #f0f4ff;
  padding: 15px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.selected-files h3 {
  margin-top: 0;
}

.selected-files ul {
  list-style: none;
  padding: 0;
  margin: 10px 0;
}

.selected-files li {
  padding: 8px;
  background-color: white;
  border-radius: 4px;
  margin-bottom: 8px;
  color: #333;
}

.btn-clear {
  background-color: #f56565;
  color: white;
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.btn-clear:hover {
  background-color: #e53e3e;
}

.button-group {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
}

.btn-next {
  background-color: #48bb78;
  color: white;
  padding: 12px 30px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
  transition: background-color 0.3s;
}

.btn-next:hover {
  background-color: #38a169;
}

/* 移动端适配 */
@media (max-width: 768px) {
  .upload-content {
    padding: 20px;
  }

  h1 {
    font-size: 22px;
  }

  .drag-drop-area {
    padding: 20px;
  }

  .button-group {
    flex-direction: column;
  }

  .btn-next {
    width: 100%;
  }
}
</style>
