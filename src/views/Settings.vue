<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useSettingsStore } from '../stores/settings'
import { useHistoryStore } from '../stores/history'
import { useSafeAuthStore } from '../auth/authWrapper'

const settingsStore = useSettingsStore()
const historyStore = useHistoryStore()
const authStore = useSafeAuthStore()

const currentLimit = ref(settingsStore.historyLimit)

// Determine max limit based on membership
const maxAllowed = computed(() => {
  if (!authStore.isLoggedIn) return 3
  
  const type = authStore.activeMembership?.membershipType
  if (type === 'ultra') return 100
  if (type === 'pro') return 10
  return 3 // standard/free
})

const membershipLabel = computed(() => {
  if (!authStore.isLoggedIn) return '游客 (Free)'
  const type = authStore.activeMembership?.membershipType
  if (type === 'ultra') return 'Ultra 会员'
  if (type === 'pro') return 'Pro 会员'
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
    if(confirm('确定要清空所有转换历史吗？')) {
        historyStore.clearHistory()
    }
}

</script>

<template>
  <div class="max-w-4xl mx-auto px-4 py-8">
    <h1 class="text-2xl font-bold mb-6 text-gray-800">设置</h1>

    <div class="bg-white shadow rounded-lg p-6 mb-6">
      <h2 class="text-xl font-semibold mb-4 border-b pb-2">通用设置</h2>
      
      <div class="grid gap-6">
        <!-- History Limit -->
        <div class="flex flex-col gap-2">
            <div class="flex justify-between items-center">
                <label class="font-medium text-gray-700">保存历史记录条数</label>
                <span class="text-sm text-gray-500">
                    当前权益: {{ membershipLabel }} (上限 {{ maxAllowed }} 条)
                </span>
            </div>
            <div class="flex items-center gap-4">
                <input 
                    type="number" 
                    v-model.number="currentLimit"
                    @change="saveLimit"
                    min="0"
                    :max="maxAllowed"
                    class="border border-gray-300 rounded px-3 py-2 w-32 focus:ring-blue-500 focus:border-blue-500"
                />
                <span class="text-sm text-gray-500">
                    设置范围: 0 - {{ maxAllowed }}
                </span>
            </div>
            <p class="text-xs text-gray-400">
                超出限制的历史记录将被自动清除。
            </p>
        </div>

        <div class="border-t pt-4">
            <button 
                @click="clearHistory"
                class="bg-red-50 text-red-600 border border-red-200 px-4 py-2 rounded hover:bg-red-100 transition-colors"
            >
                清空转换历史
            </button>
        </div>
      </div>
    </div>
  </div>
</template>
