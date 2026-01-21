import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUploadStore = defineStore('upload', () => {
  const files = ref<File[]>([])
  const uploadedFiles = ref<any[]>([])
  const currentStep = ref(1)

  const addFiles = (newFiles: File[]) => {
    files.value = newFiles
  }

  const clearFiles = () => {
    files.value = []
  }

  const setStep = (step: number) => {
    currentStep.value = step
  }

  return {
    files,
    uploadedFiles,
    currentStep,
    addFiles,
    clearFiles,
    setStep
  }
})
