<script setup lang="ts">
import { computed, watch, onUnmounted } from 'vue'

interface DownloadProgressData {
  downloaded: number
  total: number
  percentage: number
}

const props = defineProps<{
  isVisible: boolean
  title: string
  detail?: string
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

const toggleScrollLock = (locked: boolean) => {
  document.body.classList.toggle('overflow-hidden', locked)
}

watch(
  () => props.isVisible,
  (visible) => {
    toggleScrollLock(visible)
  },
  { immediate: true }
)

onUnmounted(() => {
  toggleScrollLock(false)
})
</script>

<template>
  <div v-if="isVisible" class="fixed inset-0 bg-slate-900/70 backdrop-blur-sm flex items-center justify-center z-50 px-4">
    <div class="bg-white rounded-2xl shadow-2xl p-6 w-full max-w-xl border border-slate-100">
      <div class="flex items-start justify-between mb-4">
        <div>
          <p class="text-xs uppercase tracking-[0.2em] text-slate-500">准备必要组件</p>
          <h3 class="text-2xl font-bold text-slate-900 mt-1">{{ title }}</h3>
          <p v-if="detail" class="text-sm text-slate-500 mt-1">当前任务：{{ detail }}</p>
        </div>
        <div class="w-10 h-10 rounded-full bg-indigo-50 border border-indigo-100 flex items-center justify-center text-indigo-600 text-lg">⬇️</div>
      </div>
      
      <div class="mb-3">
        <div class="w-full bg-slate-100 rounded-full h-3 overflow-hidden">
          <div 
            class="bg-indigo-600 h-full transition-all duration-300 ease-out"
            :style="{ width: progressWidth }"
          ></div>
        </div>
        <div class="flex justify-between mt-2 text-xs text-slate-600">
          <span>{{ formatBytes(progress.downloaded) }} / {{ formatBytes(progress.total) }}</span>
          <span>{{ progress.percentage.toFixed(2) }}%</span>
        </div>
      </div>
      
      <p class="text-sm text-slate-500 text-center">请稍候，正在下载并安装所需组件，完成后即可继续使用。</p>
    </div>
  </div>
</template>

<style scoped>
</style>
