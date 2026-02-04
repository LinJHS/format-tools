<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useHistoryStore } from '../stores/history'
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener'
import { useSafeAuthStore } from '../auth/authWrapper'

const router = useRouter()
const authStore = useSafeAuthStore()
const historyStore = useHistoryStore()


// Filter records for logged out users or based on membership (UI only)
// The actual storage pruning is done in the store/logic, but for display we can also be selective
const displayRecords = computed(() => {
    // If not logged in (including open source version), only show 1 latest record
    if (!authStore.isLoggedIn) {
        return historyStore.records.slice(0, 1)
    }
    return historyStore.records
})

const openFile = async (path?: string) => {
  if (!path) return
  try {
    await openPath(path)
  } catch (e) {
    console.error('Failed to open file', e)
    alert('无法打开文件: ' + path)
  }
}

const openFolder = async (path?: string) => {
  if (!path) return
  try {
    await revealItemInDir(path)
  } catch (e) {
    console.error('Failed to open folder', e)
    alert('无法打开文件夹: ' + path)
  }
}

const formatDate = (ts: number) => {
  return new Date(ts).toLocaleString()
}

const goBack = () => {
  router.back()
}
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 py-8">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <div class="w-20">
        <button @click="goBack" class="flex items-center gap-2 text-blue-600 hover:text-blue-700 font-medium">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
          </svg>
          返回
        </button>
      </div>
      <h1 class="text-2xl font-bold text-gray-900">转换历史</h1>
      <span class="w-20 text-sm text-gray-500">共 {{ displayRecords.length }} 条记录</span>
    </div>
    

    <div v-if="displayRecords.length === 0" class="text-center py-12 bg-white rounded-lg shadow-sm">
      <p class="text-gray-500">暂无转换历史</p>
    </div>

    <div v-else class="space-y-4">
      <div v-for="record in displayRecords" :key="record.id" 
           class="bg-white p-4 rounded-lg shadow-sm border border-gray-100 hover:shadow-md transition-shadow flex justify-between items-center">
        
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
             <span class="font-semibold text-gray-800 text-lg truncate" :title="record.fileName">
                {{ record.fileName }}
             </span>
             <span v-if="record.status === 'success'" class="bg-green-100 text-green-700 text-xs px-2 py-0.5 rounded">成功</span>
             <span v-else class="bg-red-100 text-red-700 text-xs px-2 py-0.5 rounded">失败</span>
          </div>
          
          <div class="text-sm text-gray-500 flex flex-col sm:flex-row sm:gap-4">
             <span>📅 {{ formatDate(record.date) }}</span>
             <span>📝 模板: {{ record.templateName }}</span>
          </div>
          
          <div v-if="record.errorMessage" class="text-sm text-red-500 mt-1">
             原因: {{ record.errorMessage }}
          </div>
          <div v-if="record.outputPath" class="text-xs text-gray-400 mt-1 truncate" :title="record.outputPath">
             {{ record.outputPath }}
          </div>
        </div>

        <div class="flex gap-2 ml-4">
            <button 
                v-if="record.status === 'success' && record.outputPath"
                @click="openFile(record.outputPath)"
                class="bg-blue-50 text-blue-600 border border-blue-200 px-3 py-1.5 rounded text-sm hover:bg-blue-100"
            >
                打开文件
            </button>
            <button 
                v-if="record.status === 'success' && record.outputPath"
                @click="openFolder(record.outputPath)"
                class="bg-purple-50 text-purple-600 border border-purple-200 px-3 py-1.5 rounded text-sm hover:bg-purple-100"
            >
                打开位置
            </button>
        </div>
      </div>
    </div>
  </div>
</template>
