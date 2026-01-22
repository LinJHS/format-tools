/**
 * Auth state (used only when VITE_ENABLE_AUTH === 'true').
 */
import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

export type SubscriptionLevel = 'basic' | 'pro' | 'master'

export interface AuthPayload {
  user: any
  token: string
  orders?: any[]
  subscriptionLevel?: SubscriptionLevel
  activeUntil?: string | null
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<any | null>(null)
  const token = ref<string>('')
  const orders = ref<any[]>([])
  const subscriptionLevel = ref<SubscriptionLevel>('basic')
  const activeUntil = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string>('')

  const isLoggedIn = computed(() => Boolean(user.value && token.value))

  const setAuth = (payload: AuthPayload) => {
    user.value = payload.user
    token.value = payload.token
    orders.value = payload.orders || []
    subscriptionLevel.value = payload.subscriptionLevel || 'basic'
    activeUntil.value = payload.activeUntil || null
  }

  const setLoading = (value: boolean) => {
    loading.value = value
  }

  const setError = (message: string) => {
    error.value = message
  }

  const clearAuth = () => {
    user.value = null
    token.value = ''
    orders.value = []
    subscriptionLevel.value = 'basic'
    activeUntil.value = null
    error.value = ''
    loading.value = false
  }

  return {
    user,
    token,
    orders,
    subscriptionLevel,
    activeUntil,
    loading,
    error,
    isLoggedIn,
    setAuth,
    setLoading,
    setError,
    clearAuth
  }
})
