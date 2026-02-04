<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../stores/settings'
import { useHistoryStore } from '../stores/history'
import { useSafeAuthStore } from '../auth/authWrapper'
import ConfirmDialog from '../components/ConfirmDialog.vue'

const router = useRouter()
const settingsStore = useSettingsStore()
const historyStore = useHistoryStore()
const authStore = useSafeAuthStore()

const currentLimit = ref(settingsStore.historyLimit)
const showClearDialog = ref(false)

// Determine max limit based on membership
const maxAllowed = computed(() => {
  if (!authStore.isLoggedIn) return 3

  const type = authStore.activeMembership?.membershipType
  if (type === 'ultra') return 100
  if (type === 'pro') return 10
  return 3 // standard/free
})

const membershipLabel = computed(() => {
  if (!authStore.isLoggedIn) return '游客 (免费版)'
  const type = authStore.activeMembership?.membershipType
  if (type === 'ultra') return '大师版会员'
  if (type === 'pro') return '专业版会员'
  return '普通会员'
})

// Sync local ref with store
watch(() => settingsStore.historyLimit, (val) => {
  currentLimit.value = val
})

// Save when changes
const saveLimit = () => {
  let val = currentLimit.value
  if (val < 0) val = 0
  if (val > maxAllowed.value) val = maxAllowed.value

  currentLimit.value = val
  settingsStore.historyLimit = val

  // Also prune history immediately if limit reduced
  historyStore.prune(val)
}

const clearHistory = () => {
  showClearDialog.value = true
}

const handleConfirmClear = async () => {
  try {
    // 1. Clear local history
    historyStore.clearHistory()
    // 2. Clear remote sessions
    await invoke('clear_sessions')
  } catch (err) {
    console.error('Failed to clear sessions:', err)
  }
  showClearDialog.value = false
}

const goBack = () => {
  router.back()
}
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 py-8">
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <button @click="goBack" class="flex items-center gap-2 text-blue-600 hover:text-blue-700 font-medium">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
        </svg>
        返回
      </button>
      <h1 class="text-2xl font-bold text-gray-900">应用设置</h1>
      <div class="w-16"></div>
    </div>

    <!-- Group: Storage -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden mb-8">
      <div class="p-4 border-b border-gray-100 bg-gray-50 flex justify-between items-center">
        <h2 class="font-semibold text-gray-900">存储与记录</h2>
        <span class="text-xs px-2 py-1 bg-blue-100 text-blue-700 rounded-full font-medium">{{ membershipLabel }}</span>
      </div>

      <div class="divide-y divide-gray-100">
        <div class="p-4">
          <div class="flex justify-between items-center mb-4">
            <div>
              <label class="block font-medium text-gray-900">历史记录保留条数</label>
              <p class="text-sm text-gray-500 mt-1">设置本地保存的最大记录数量 (上限 {{ maxAllowed }} 条)</p>
            </div>
            <div class="w-24">
              <input type="number" v-model.number="currentLimit" @change="saveLimit" min="0" :max="maxAllowed"
                class="w-full text-center border-gray-200 bg-gray-50 rounded-lg focus:ring-blue-500 focus:border-blue-500 transition-shadow py-2" />
            </div>
          </div>

          <!-- Progress Bar Visual -->
          <div class="w-full bg-gray-100 rounded-full h-2 mb-2">
            <div class="bg-blue-600 h-2 rounded-full transition-all duration-500"
              :style="{ width: Math.min((currentLimit / maxAllowed) * 100, 100) + '%' }"></div>
          </div>
          <div class="flex justify-between text-xs text-gray-400">
            <span>0</span>
            <span>{{ maxAllowed }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Group: Danger Zone -->
    <div class="bg-white rounded-2xl shadow-sm border border-red-100 overflow-hidden">
      <div class="p-4">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-medium text-gray-900">清空转换历史</h3>
            <p class="text-sm text-gray-500 mt-1">此操作将删除所有本地记录，您的文件不会被删除。</p>
          </div>
          <button @click="clearHistory"
            class="px-4 py-2 bg-white border border-red-200 text-red-600 rounded-lg hover:bg-red-50 transition-colors text-sm font-medium shadow-sm">
            立即清空
          </button>
        </div>
      </div>
    </div>

    <ConfirmDialog 
      :visible="showClearDialog"
      title="清空转换历史"
      message="此操作将删除所有本地转换历史记录，您的文件不会被删除。此操作无法撤销。"
      confirmText="立即清空"
      @confirm="handleConfirmClear"
      @cancel="showClearDialog = false"
    />
  </div>
</template>
