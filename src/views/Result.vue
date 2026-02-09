<script setup lang="ts">
import { computed } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener'
import { warn as logWarn } from '@tauri-apps/plugin-log'

const uploadStore = useUploadStore()
const router = useRouter()

const results = computed(() => uploadStore.results)
const successCount = computed(() => results.value.filter(r => r.status === 'success').length)
const failCount = computed(() => results.value.filter(r => r.status === 'failed').length)
const totalCount = computed(() => results.value.length)

const goHome = () => {
  router.push('/')
}

const openFile = async (path?: string) => {
  if (!path) return
  try {
    await openPath(path)
  } catch (e) {
    logWarn(`无法打开文件: ${e}`)
  }
}

const openFolder = async (path?: string) => {
  if (!path) return
  try {
    await revealItemInDir(path)
  } catch (e) {
    logWarn(`无法打开文件夹: ${e}`)
  }
}

const openResultFolder = async () => {
  // Open folder of the first successful item, or just standard open logic
  const first = results.value.find(r => r.outputPath)
  if (first && first.outputPath) {
    openFolder(first.outputPath)
  }
}
</script>

<template>
  <div class="min-h-[calc(100vh-56px)] p-6 bg-[radial-gradient(circle_at_20%_20%,#f5f7ff,#eef2ff_40%,#e8edf8_80%)]">
    <div class="max-w-4xl mx-auto bg-white rounded-2xl p-8 shadow-[0_22px_80px_rgba(52,64,84,0.14)]">
      <!-- 头部 -->
      <div class="flex items-center justify-between gap-4 mb-8">
        <div>
          <h1 class="m-0 text-[#1f2937] text-3xl tracking-tight font-bold">转换完成</h1>
          <p class="m-0 mt-2 text-[#6b7280] text-sm">
            共 {{ totalCount }} 个文件，成功 {{ successCount }} 个，失败 {{ failCount }} 个
          </p>
        </div>
        <span class="bg-[#dcfce7] text-[#166534] px-4 py-2 rounded-lg font-semibold text-sm"
          :class="{ 'bg-red-50 text-red-600': failCount > 0 && successCount === 0, 'bg-yellow-50 text-yellow-600': failCount > 0 && successCount > 0 }">
          {{ failCount === 0 ? '全部成功' : (successCount === 0 ? '全部失败' : '部分成功') }}
        </span>
      </div>

      <!-- 结果列表 -->
      <div class="bg-[#fafbff] border border-[#e0e7ff] rounded-xl overflow-hidden mb-8">
        <div v-for="(result, index) in results" :key="index"
          class="p-4 border-b border-[#e0e7ff] last:border-b-0 flex items-center justify-between hover:bg-white transition-colors">
          <div class="flex items-center gap-3 overflow-hidden">
            <div class="w-8 h-8 rounded-full flex items-center justify-center shrink-0"
              :class="result.status === 'success' ? 'bg-green-100 text-green-600' : 'bg-red-100 text-red-600'">
              <span v-if="result.status === 'success'">✓</span>
              <span v-else>✕</span>
            </div>
            <div class="min-w-0">
              <p class="m-0 text-[#111827] font-semibold text-sm truncate">{{ result.fileName }}</p>
              <p v-if="result.status === 'failed'" class="m-0 text-red-500 text-xs truncate">{{ result.error }}</p>
              <p v-else class="m-0 text-[#6b7280] text-xs truncate font-mono">{{ result.outputPath }}</p>
            </div>
          </div>

          <div class="flex gap-2 shrink-0" v-if="result.status === 'success'">
            <button @click="openFile(result.outputPath)"
              class="px-3 py-1.5 rounded-lg text-xs font-semibold text-[#3b82f6] bg-blue-50 hover:bg-blue-100 transition-colors">打开文件</button>
            <button @click="openFolder(result.outputPath)"
              class="px-3 py-1.5 rounded-lg text-xs font-semibold text-[#8b5cf6] bg-purple-50 hover:bg-purple-100 transition-colors">文件夹</button>
          </div>
        </div>
        <div v-if="results.length === 0" class="p-8 text-center text-gray-400">
          无结果信息
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex items-center justify-between gap-4 mt-8">
        <button
          class="px-6 py-2.5 rounded-lg text-sm font-semibold text-[#6b7280] bg-[#f3f4f6] cursor-pointer transition-all hover:bg-[#e5e7eb]"
          @click="goHome">
          ← 回到首页
        </button>
        <div class="flex gap-3">
          <button v-if="successCount > 0"
            class="px-6 py-2.5 rounded-lg text-sm font-bold text-white cursor-pointer transition-all shadow-[0_12px_30px_rgba(168,85,247,0.25)] hover:-translate-y-0.5 hover:shadow-[0_14px_34px_rgba(168,85,247,0.3)] bg-[linear-gradient(90deg,#a855f7,#9333ea)]"
            @click="openResultFolder">
            打开输出目录
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 样式主要由 Tailwind 提供；此处留空以便微调 */
</style>
