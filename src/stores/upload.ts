import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ConversionResult {
  fileName: string
  outputPath?: string
  status: 'success' | 'failed'
  error?: string
}

export const useUploadStore = defineStore('upload', () => {
  const files = ref<File[]>([])
  const uploadedFiles = ref<any[]>([])
  const currentStep = ref(1)
  const mode = ref<'file' | 'text'>('file')
  const markdownText = ref('')
  const preparedInput = ref<any>(null) // Deprecated: use preparedInputs
  const preparedInputs = ref<any[]>([]) // New: Array of prepared inputs
  const outputPath = ref<string>('') // Deprecated: use results
  const results = ref<ConversionResult[]>([]) // New: Array of conversion results

  const addFiles = (newFiles: File[]) => {
    // Append unique files
    const existingNames = new Set(files.value.map(f => f.name))
    const uniqueNewFiles = newFiles.filter(f => !existingNames.has(f.name))
    files.value = [...files.value, ...uniqueNewFiles]
  }

  const removeFile = (fileName: string) => {
    files.value = files.value.filter(f => f.name !== fileName)
  }

  const clearFiles = () => {
    files.value = []
    preparedInputs.value = []
  }

  const setMode = (value: 'file' | 'text') => {
    mode.value = value
  }

  const setMarkdownText = (content: string) => {
    markdownText.value = content
  }

  const setPreparedInput = (payload: any) => {
    preparedInput.value = payload
    // Also set single item array for compatibility if needed, 
    // but logic should ideally use setPreparedInputs
    preparedInputs.value = [payload]
  }

  const setPreparedInputs = (payloads: any[]) => {
    preparedInputs.value = payloads
    // Sync first item to legacy ref if at least one exists
    if (payloads.length > 0) {
      preparedInput.value = payloads[0]
    } else {
      preparedInput.value = null
    }
  }

  const setOutputPath = (path: string) => {
    outputPath.value = path
  }

  const setResults = (newResults: ConversionResult[]) => {
    results.value = newResults
    // visual compatibility: if any success, set last one as outputPath
    const lastSuccess = newResults.filter(r => r.status === 'success').pop()
    if (lastSuccess && lastSuccess.outputPath) {
      outputPath.value = lastSuccess.outputPath
    }
  }

  const setStep = (step: number) => {
    currentStep.value = step
  }

  return {
    files,
    uploadedFiles,
    currentStep,
    mode,
    markdownText,
    preparedInput,
    preparedInputs,
    outputPath,
    results,
    addFiles,
    removeFile,
    clearFiles,
    setMode,
    setMarkdownText,
    setPreparedInput,
    setPreparedInputs,
    setOutputPath,
    setResults,
    setStep
  }
})
