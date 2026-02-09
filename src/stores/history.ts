import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { error as logError } from '@tauri-apps/plugin-log'

export interface ConversionRecord {
  id: string
  date: number
  fileName: string
  templateName: string
  outputPath?: string
  status: 'success' | 'failed'
  errorMessage?: string
}

export const useHistoryStore = defineStore('history', () => {
  const records = ref<ConversionRecord[]>([])

  // Load
  const saved = localStorage.getItem('conversion_history')
  if (saved) {
    try {
      records.value = JSON.parse(saved)
    } catch (e) {
      logError(`Failed to parse history: ${e}`)
    }
  }

  // Persist
  watch(records, (val) => {
    localStorage.setItem('conversion_history', JSON.stringify(val))
  }, { deep: true })

  const addRecord = (record: ConversionRecord) => {
    records.value.unshift(record)
  }

  const clearHistory = () => {
    records.value = []
  }

  /**
   * Prune records to keep only the latest N
   */
  const prune = (limit: number) => {
    if (records.value.length > limit) {
      records.value = records.value.slice(0, limit)
    }
  }

  return {
    records,
    addRecord,
    clearHistory,
    prune
  }
})
