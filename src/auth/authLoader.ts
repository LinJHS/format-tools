import type { Router, RouteRecordRaw } from 'vue-router'

// This loader safely checks for a private auth module without failing builds when absent.
// The private module should live in src/auth-private/index.ts and export `{ routes: RouteRecordRaw[] }`.
const authModules = import.meta.glob('../auth-private/index.ts')

export async function loadAuthRoutes(router: Router) {
  // Env guard: only attempt when explicitly enabled
  if (import.meta.env.VITE_ENABLE_AUTH !== 'true') return

  const entries = Object.entries(authModules)
  if (!entries.length) return

  try {
    const mod = (await entries[0][1]()) as { routes?: RouteRecordRaw[] }
    mod.routes?.forEach((route) => router.addRoute(route))
  } catch (err) {
    console.warn('Auth module load failed, skipping auth routes:', err)
  }
}

export async function initAuthState() {
  // Env guard: only attempt when explicitly enabled
  if (import.meta.env.VITE_ENABLE_AUTH !== 'true') return

  try {
    const { useAuthStore } = await import('../auth-private/stores/auth')
    const { fetchUserInfo, fetchMembership } = await import('../auth-private/api/emasAuth')
    const authStore = useAuthStore()

    // Try to load persisted auth
    const hasPersistedAuth = authStore.loadPersistedAuth()

    if (hasPersistedAuth && authStore.token) {
      // Validate and refresh user info
      const userInfoResult = await fetchUserInfo(authStore.token)

      if (userInfoResult.success && userInfoResult.user) {
        // Update user info
        authStore.setAuth({
          user: userInfoResult.user,
          token: authStore.token,
          memberships: authStore.memberships
        })

        // Refresh membership info
        const membershipResult = await fetchMembership(authStore.token)
        if (membershipResult.success && membershipResult.memberships) {
          authStore.setMemberships(membershipResult.memberships)
        }
      } else {
        // Token is invalid or expired, clear auth
        console.warn('Token validation failed, clearing auth')
        authStore.clearAuth()
      }
    }
  } catch (err) {
    console.warn('Auth state initialization failed:', err)
  }
}
