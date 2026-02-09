import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { error as logError } from '@tauri-apps/plugin-log'

export const useSettingsStore = defineStore('settings', () => {
  // Default to 100, but effective limit will be clamped by logic
  const historyLimit = ref<number>(3) 
  const textExportPath = ref<string | null>(null)

  // Load from local storage
  const savedSettings = localStorage.getItem('app_settings')
  if (savedSettings) {
    try {
      const parsed = JSON.parse(savedSettings)
      if (typeof parsed.historyLimit === 'number') {
        historyLimit.value = parsed.historyLimit
      }
      if (typeof parsed.textExportPath === 'string') {
        textExportPath.value = parsed.textExportPath
      }
    } catch (e) {
      logError(`Failed to parse settings: ${e}`)
    }
  }

  // Persist
  watch([historyLimit, textExportPath], ([limit, path]) => {
    localStorage.setItem('app_settings', JSON.stringify({ 
      historyLimit: limit,
      textExportPath: path 
    }))
  })

  return {
    historyLimit,
    textExportPath
  }
})
