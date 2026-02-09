import type { Router, RouteRecordRaw } from 'vue-router'
import { warn as logWarn } from '@tauri-apps/plugin-log'

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
    logWarn(`Auth module load failed, skipping auth routes: ${err}`)
  }
}

export async function initAuthState() {
  // Env guard: only attempt when explicitly enabled
  if (import.meta.env.VITE_ENABLE_AUTH !== 'true') return

  const entries = Object.entries(authModules)
  if (!entries.length) return

  try {
    const mod = (await entries[0][1]()) as { initializeAuthState?: () => Promise<void> }
    await mod.initializeAuthState?.()
  } catch (err) {
    logWarn(`Auth state initialization failed: ${err}`)
  }
}
