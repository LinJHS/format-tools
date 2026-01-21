import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUploadStore = defineStore('upload', () => {
  const files = ref<File[]>([])
  const uploadedFiles = ref<any[]>([])
  const currentStep = ref(1)
  const mode = ref<'file' | 'text'>('file')
  const markdownText = ref('')
  const preparedInput = ref<any>(null)
  const outputPath = ref<string>('')

  const addFiles = (newFiles: File[]) => {
    files.value = newFiles
  }

  const clearFiles = () => {
    files.value = []
  }

  const setMode = (value: 'file' | 'text') => {
    mode.value = value
  }

  const setMarkdownText = (content: string) => {
    markdownText.value = content
  }

  const setPreparedInput = (payload: any) => {
    preparedInput.value = payload
  }

  const setOutputPath = (path: string) => {
    outputPath.value = path
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
    outputPath,
    addFiles,
    clearFiles,
    setMode,
    setMarkdownText,
    setPreparedInput,
    setOutputPath,
    setStep
  }
})
