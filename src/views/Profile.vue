<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { openUrl } from '@tauri-apps/plugin-opener'
import { useAuthStore } from '../auth-private/stores/auth'
import { LINKS } from '../config/links'

const router = useRouter()
const authStore = useAuthStore()

const authEnabled = import.meta.env.VITE_ENABLE_AUTH === 'true'
const isLoggedIn = computed(() => authStore.isLoggedIn)
const userName = computed(() => authStore.user?.displayName || '未登录')
const userEmail = computed(() => authStore.user?.email || '')
const userAvatar = computed(() => {
  const avatar = authStore.user?.avatar || ''
  if (avatar.startsWith('https://linjhs.com')) {
    return avatar
  } else if (avatar) {
    return `https://linjhs.com${avatar}`
  }
  return '' // Empty string for fallback to SVG
})
const userPhone = computed(() => authStore.user?.phone || '')

const goToLogin = async () => {
  if (authEnabled) {
    router.push('/login')
  } else {
    await openUrl(LINKS.login)
  }
}

const goToShop = async () => {
  await openUrl(LINKS.shop)
}

const handleLogout = () => {
  authStore.clearAuth()
}

const menuItems = [
  { icon: '👤', title: '个人信息', description: '查看和管理您的账户信息' },
  { icon: '💎', title: '会员中心', description: '升级会员，解锁更多功能' },
  { icon: '📊', title: '转换历史', description: '查看您的文档转换记录' },
  { icon: '⚙️', title: '设置', description: '自定义您的使用偏好' },
  { icon: 'ℹ️', title: '关于我们', description: '了解格式匠的更多信息' },
]
</script>

<template>
  <div class="profile-container min-h-screen bg-gray-50 py-6">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div v-if="!authEnabled" class="bg-white rounded-2xl shadow-lg p-10 text-center text-gray-700">
        <h2 class="text-xl font-bold mb-3">当前为开源版本</h2>
        <p class="text-sm">登录 / 注册功能未启用。</p>
      </div>

      <template v-else>
        <!-- User Card -->
        <div class="bg-white rounded-2xl shadow-lg overflow-hidden mb-8">
          <div class="bg-linear-to-r from-purple-600 to-indigo-600 h-20"></div>
          <div class="px-6 pb-6">
            <div class="flex items-end -mt-16">
              <div class="w-28 h-28 rounded-full bg-white shadow-lg border-4 border-white overflow-hidden">
                <img v-if="userAvatar" :src="userAvatar" alt="User Avatar" class="w-full h-full object-cover" />
                <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-full h-full text-gray-400">
                  <g fill="#888888" fill-opacity="0" stroke="#888888" stroke-linecap="round" stroke-linejoin="round"
                    stroke-width="2">
                    <path stroke-dasharray="22"
                      d="M12 5c1.66 0 3 1.34 3 3c0 1.66 -1.34 3 -3 3c-1.66 0 -3 -1.34 -3 -3c0 -1.66 1.34 -3 3 -3Z">
                      <animate fill="freeze" attributeName="stroke-dashoffset" dur="0.5s" values="22;0" />
                      <animate fill="freeze" attributeName="fill-opacity" begin="1.1s" dur="0.4s" to="1" />
                    </path>
                    <path stroke-dasharray="38" stroke-dashoffset="38" d="M12 14c4 0 7 2 7 3v2h-14v-2c0 -1 3 -3 7 -3Z">
                      <animate fill="freeze" attributeName="stroke-dashoffset" begin="0.5s" dur="0.5s" to="0" />
                      <animate fill="freeze" attributeName="fill-opacity" begin="1.1s" dur="0.4s" to="1" />
                    </path>
                  </g>
                </svg>
              </div>
            </div>

            <div v-if="!isLoggedIn" class="text-center">
              <h2 class="text-xl font-bold text-gray-900 mb-3">您还未登录</h2>
              <p class="text-gray-600 mb-5 text-sm">登录后可以查看会员信息、转换历史等更多功能</p>
              <div class="flex gap-3 justify-center">
                <button @click="goToLogin"
                  class="bg-purple-600 text-white px-6 py-2.5 rounded-lg font-semibold hover:bg-purple-700 transition-colors">
                  立即登录
                </button>
                <button @click="async () => await openUrl(LINKS.signup)"
                  class="bg-gray-200 text-gray-700 px-6 py-2.5 rounded-lg font-semibold hover:bg-gray-300 transition-colors">
                  注册账号
                </button>
              </div>
            </div>

            <div v-else>
              <h2 class="text-xl font-bold text-gray-900 mb-2">{{ userName }}</h2>
              <p class="text-gray-600 mb-3 text-sm">{{ userEmail }}</p>
              <p class="text-gray-600 mb-3 text-sm">{{ userPhone }}</p>
              <div class="mt-4 flex gap-3">
                <button @click="goToShop"
                  class="bg-purple-600 text-white px-4 py-2 rounded-lg font-semibold hover:bg-purple-700 transition-colors">
                  会员中心
                </button>
                <button @click="handleLogout"
                  class="bg-gray-200 text-gray-700 px-4 py-2 rounded-lg font-semibold hover:bg-gray-300 transition-colors">
                  退出登录
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Menu Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div v-for="(item, index) in menuItems" :key="index"
            class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all cursor-pointer p-5 transform hover:-translate-y-1"
            @click="item.title === '会员中心' ? goToShop() : null">
            <div class="flex items-start">
              <div class="text-3xl mr-3">{{ item.icon }}</div>
              <div class="flex-1">
                <h3 class="text-base font-semibold text-gray-900 mb-1">
                  {{ item.title }}
                </h3>
                <p class="text-gray-600 text-sm">
                  {{ item.description }}
                </p>
              </div>
              <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>
          </div>
        </div>

        <!-- Quick Actions for Non-logged Users -->
        <div v-if="!isLoggedIn"
          class="mt-6 bg-linear-to-r from-purple-50 to-indigo-50 rounded-2xl p-6 border border-purple-100">
          <h3 class="text-lg font-bold text-gray-900 mb-3">快速链接</h3>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
            <a :href="LINKS.login" target="_blank"
              class="flex items-center justify-center bg-white px-5 py-3 rounded-lg hover:shadow-md transition-all">
              <span class="font-semibold text-gray-700">登录账号</span>
              <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
              </svg>
            </a>
            <a :href="LINKS.signup" target="_blank"
              class="flex items-center justify-center bg-white px-5 py-3 rounded-lg hover:shadow-md transition-all">
              <span class="font-semibold text-gray-700">注册账号</span>
              <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
              </svg>
            </a>
            <a :href="LINKS.shop" target="_blank"
              class="flex items-center justify-center bg-white px-5 py-3 rounded-lg hover:shadow-md transition-all">
              <span class="font-semibold text-gray-700">购买会员</span>
              <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
              </svg>
            </a>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped></style>
