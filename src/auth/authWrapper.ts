import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDummyAuthStore = defineStore('auth-dummy', () => {
  const isLoggedIn = ref(false)
  const user = ref(null)
  const token = ref('')
  const activeMembership = ref(null)
  const isUltra = ref(false)
  
  function setMemberships() {}
  function logout() {}
  
  return { isLoggedIn, user, token, activeMembership, isUltra, setMemberships, logout }
})

// Eagerly load the store definition if it exists
// Using glob ensures no build error if file is missing
// The key must match the exact relative path from this file
const authStoreModules = import.meta.glob('../auth-private/stores/auth.ts', { eager: true })
const aiServiceModules = import.meta.glob('../auth-private/services/aiFormatService.ts', { eager: true })

export function useSafeAuthStore() {
  const path = '../auth-private/stores/auth.ts'
  if (authStoreModules[path] && import.meta.env.VITE_ENABLE_AUTH === 'true') {
    return (authStoreModules[path] as any).useAuthStore()
  }
  return useDummyAuthStore()
}

export function getSafeAIFormatService() {
  const path = '../auth-private/services/aiFormatService.ts'
  if (aiServiceModules[path] && import.meta.env.VITE_ENABLE_AUTH === 'true') {
     return aiServiceModules[path] as any
  }
  return null
}
