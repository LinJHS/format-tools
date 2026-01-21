<script setup lang="ts">
import { ref, computed } from 'vue'

interface DownloadProgressData {
  downloaded: number
  total: number
  percentage: number
}

const props = defineProps<{
  isVisible: boolean
  title: string
  progress: DownloadProgressData
}>()

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

const progressWidth = computed(() => {
  return props.progress.percentage.toFixed(2) + '%'
})
</script>

<template>
  <div v-if="isVisible" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl p-6 w-96">
      <h3 class="text-xl font-semibold mb-4">{{ title }}</h3>
      
      <div class="mb-4">
        <div class="w-full bg-gray-200 rounded-full h-4 overflow-hidden">
          <div 
            class="bg-blue-600 h-full transition-all duration-300 ease-out"
            :style="{ width: progressWidth }"
          ></div>
        </div>
        <div class="flex justify-between mt-2 text-sm text-gray-600">
          <span>{{ formatBytes(progress.downloaded) }} / {{ formatBytes(progress.total) }}</span>
          <span>{{ progress.percentage.toFixed(2) }}%</span>
        </div>
      </div>
      
      <p class="text-sm text-gray-500 text-center">请稍候，正在下载中...</p>
    </div>
  </div>
</template>

<style scoped>
</style>
