/**
 * MPServerless client wrapper
 *
 * - Only enabled when VITE_ENABLE_AUTH === 'true'
 * - Uses environment-provided EMAS credentials
 * - Falls back to fetch-based request adapter when `my` is unavailable (desktop/web)
 */
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore: the SDK has no types bundled
import MPServerless from '@alicloud/mpserverless-sdk'

let cachedClient: any

const fetchRequest = async (options: any) => {
  const { url, method = 'GET', headers, data } = options || {}
  const init: RequestInit = {
    method,
    headers,
    body: method && method.toUpperCase() !== 'GET' && data ? JSON.stringify(data) : undefined
  }
  const res = await fetch(url, init)
  const payload = await res.json().catch(() => undefined)
  return { status: res.status, data: payload }
}

function ensureEnv() {
  const appId = import.meta.env.VITE_EMAS_APP_ID
  const spaceId = import.meta.env.VITE_EMAS_SPACE_ID
  const clientSecret = import.meta.env.VITE_EMAS_CLIENT_SECRET
  const endpoint = import.meta.env.VITE_EMAS_ENDPOINT || 'https://api.next.bspapp.com'

  if (!appId || !spaceId || !clientSecret) {
    throw new Error('EMAS credentials missing: VITE_EMAS_APP_ID / VITE_EMAS_SPACE_ID / VITE_EMAS_CLIENT_SECRET')
  }

  return { appId, spaceId, clientSecret, endpoint }
}

export function getMpServerless() {
  if (import.meta.env.VITE_ENABLE_AUTH !== 'true') {
    throw new Error('Auth is disabled via environment flag')
  }

  if (cachedClient) return cachedClient

  const { appId, spaceId, clientSecret, endpoint } = ensureEnv()
  // Prefer Alipay mini-program adapters when available, fallback to fetch for desktop
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const globalMy: any = (globalThis as any).my

  cachedClient = new MPServerless(
    {
      uploadFile: globalMy?.uploadFile,
      request: globalMy?.request || fetchRequest,
      getAuthCode: globalMy?.getAuthCode
    },
    {
      appId,
      spaceId,
      clientSecret,
      endpoint
    }
  )

  return cachedClient
}

export async function invokeCloudFunction<T = any>(name: string, payload: Record<string, unknown>) {
  const client = getMpServerless()
  const res = await client.function.invoke(name, payload)
  if (res?.success === false) {
    throw new Error(res?.error || 'Invoke failed')
  }
  return (res?.result ?? res) as T
}
