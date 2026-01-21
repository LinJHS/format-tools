<script setup lang="ts">
import { computed } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { openPath } from '@tauri-apps/plugin-opener'

const uploadStore = useUploadStore()
const router = useRouter()

const outputPath = computed(() => uploadStore.outputPath)

const fileName = computed(() => {
  if (!outputPath.value) return ''
  const parts = outputPath.value.split(/[\\/]/)
  return parts[parts.length - 1]
})

const folderPath = computed(() => {
  if (!outputPath.value) return ''
  const parts = outputPath.value.split(/[\\/]/)
  parts.pop()
  return parts.join('\\')
})

const goHome = () => {
  router.push('/')
}

const openFile = async () => {
  if (!outputPath.value) return
  try {
    await openPath(outputPath.value)
  } catch (e) {
    console.warn('无法打开文件:', e)
  }
}

const openFolder = async () => {
  if (!folderPath.value) return
  try {
    await openPath(folderPath.value)
  } catch (e) {
    console.warn('无法打开文件夹:', e)
  }
}
</script>

<template>
  <div class="min-h-[calc(100vh-56px)] p-6 bg-[radial-gradient(circle_at_20%_20%,#f5f7ff,#eef2ff_40%,#e8edf8_80%)]">
    <div class="max-w-3xl mx-auto bg-white rounded-2xl p-8 shadow-[0_22px_80px_rgba(52,64,84,0.14)]">
      <!-- 头部 -->
      <div class="flex items-center justify-between gap-4 mb-8">
        <div>
          <h1 class="m-0 text-[#1f2937] text-3xl tracking-tight font-bold">转换完成</h1>
          <p class="m-0 mt-2 text-[#6b7280] text-sm">文档已经成功生成</p>
        </div>
        <span class="bg-[#dcfce7] text-[#166534] px-4 py-2 rounded-lg font-semibold text-sm">成功</span>
      </div>

      <!-- 输出信息 -->
      <div class="bg-[#fafbff] border border-[#e0e7ff] rounded-xl p-6 mb-8">
        <div class="mb-4">
          <label class="block text-xs font-semibold text-[#6b7280] uppercase tracking-wide mb-2">文件名</label>
          <div class="flex items-center gap-2">
            <p class="m-0 text-[#111827] text-base font-semibold">{{ fileName || '未知' }}</p>
          </div>
        </div>
        <div>
          <label class="block text-xs font-semibold text-[#6b7280] uppercase tracking-wide mb-2">保存位置</label>
          <div class="flex items-center gap-2">
            <p class="m-0 text-[#4b5563] text-sm font-mono break-all">{{ folderPath || '暂无' }}</p>
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex items-center justify-between gap-4 mt-8">
        <button 
          class="px-6 py-2.5 rounded-lg text-sm font-semibold text-[#6b7280] bg-[#f3f4f6] cursor-pointer transition-all hover:bg-[#e5e7eb]"
          @click="goHome"
        >
          ← 回到首页
        </button>
        <div class="flex gap-3">
          <button 
            class="px-6 py-2.5 rounded-lg text-sm font-bold text-white cursor-pointer transition-all shadow-[0_12px_30px_rgba(59,130,246,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(59,130,246,0.3)] disabled:opacity-60 disabled:cursor-not-allowed disabled:shadow-none bg-[linear-gradient(90deg,_#3b82f6,_#2563eb)]"
            @click="openFile"
          >
            打开文件
          </button>
          <button 
            class="px-6 py-2.5 rounded-lg text-sm font-bold text-white cursor-pointer transition-all shadow-[0_12px_30px_rgba(168,85,247,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(168,85,247,0.3)] disabled:opacity-60 disabled:cursor-not-allowed disabled:shadow-none bg-[linear-gradient(90deg,_#a855f7,_#9333ea)]"
            @click="openFolder"
          >
            打开文件夹
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 样式主要由 Tailwind 提供；此处留空以便微调 */
</style>
