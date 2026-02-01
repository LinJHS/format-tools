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
const userAvatar = computed(() => {
  const avatar = authStore.user?.avatar || ''
  if (avatar.startsWith('https://linjhs.com')) {
    return avatar
  } else if (avatar) {
    return `https://linjhs.com${avatar}`
  }
  return ''
})

const activeMembership = computed(() => authStore.activeMembership)

const membershipBadge = computed(() => {
  if (!activeMembership.value) return null
  if (activeMembership.value.membershipType === 'ultra') {
    return { label: '大师版', color: 'bg-gradient-to-r from-amber-400 to-amber-600' }
  }
  if (activeMembership.value.membershipType === 'pro') {
    return { label: '专业版', color: 'bg-gradient-to-r from-blue-500 to-blue-600' }
  }
  return null
})

const buttonLabel = computed(() => {
  return activeMembership.value ? '续费会员' : '购买会员'
})

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

const navigateTo = (path: string) => {
  router.push(path)
}

const handleLogout = async () => {
  authStore.clearAuth()
  router.push('/profile')
}
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
          <!-- Header (Shared) -->
          <div class="bg-blue-600 h-24 flex items-center px-6">
            <!-- Spacer for avatar alignment -->
            <div class="w-28 mr-4 shrink-0"></div>
            <!-- Username: Top Right (White Text) -->
            <div v-if="isLoggedIn" class="flex items-center gap-3">
              <div v-if="membershipBadge" :class="`${membershipBadge.color} px-3 py-1 rounded-full text-white font-bold text-sm`">
                {{ membershipBadge.label }}
              </div>
              <h2 class="text-2xl font-bold text-white leading-tight">{{ userName }}</h2>
            </div>
          </div>

          <!-- Body Content -->
          <div class="px-6 pb-6">
            <!-- Avatar Row: Overlaps header (-mt-14 aligns center 7rem avatar on line) -->
            <div class="flex items-end -mt-14 relative z-10">
              <div class="w-28 h-28 rounded-full bg-white shadow-lg border-4 border-white overflow-hidden shrink-0">
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

              <!-- Action Buttons (Logged In): Bottom Right -->
              <div v-if="isLoggedIn" class="ml-4 mb-2">
                <button @click="goToShop"
                  class="group relative overflow-hidden bg-linear-to-r from-violet-600 to-fuchsia-600 text-white px-4 py-1.5 rounded-full font-semibold shadow-lg shadow-purple-500/30 hover:shadow-purple-500/50 hover:scale-105 transition-all duration-300 text-xs">
                  <span class="relative z-10 flex items-center gap-1">
                    <svg class="w-3 h-3 transition-transform group-hover:rotate-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
                    {{ buttonLabel }}
                  </span>
                </button>
              </div>
            </div>

            <!-- Logged Out Prompt -->
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
          </div>
        </div>

        <!-- Menu List -->
        <div class="bg-white rounded-2xl shadow-lg overflow-hidden">
          <div class="divide-y divide-gray-200">
            <!-- Item: Personal Info -->
            <div
              class="hover:bg-gray-50 transition-all cursor-pointer p-4 flex items-center"
              @click="navigateTo('/personal-info')">
              <div class="text-gray-500 mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
              <div class="flex-1 font-medium text-gray-900">个人信息</div>
              <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>

            <!-- Item: Member Center -->
            <div
              class="hover:bg-gray-50 transition-all cursor-pointer p-4 flex items-center"
              @click="navigateTo('/membership-center')">
              <div class="text-gray-500 mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
                </svg>
              </div>
              <div class="flex-1 font-medium text-gray-900">会员中心</div>
              <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>

            <!-- Item: History -->
            <div
              class="hover:bg-gray-50 transition-all cursor-pointer p-4 flex items-center">
              <div class="text-gray-500 mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <div class="flex-1 font-medium text-gray-900">转换历史</div>
              <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>

            <!-- Item: Settings -->
            <div
              class="hover:bg-gray-50 transition-all cursor-pointer p-4 flex items-center">
              <div class="text-gray-500 mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </div>
              <div class="flex-1 font-medium text-gray-900">设置</div>
              <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
              </svg>
            </div>

            <!-- Item: About Us -->
            <div
              class="hover:bg-gray-50 transition-all cursor-pointer p-4 flex items-center">
              <div class="text-gray-500 mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                  stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <div class="flex-1 font-medium text-gray-900">关于我们</div>
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

