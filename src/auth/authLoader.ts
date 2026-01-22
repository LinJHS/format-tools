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
