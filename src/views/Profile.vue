<script setup lang="ts">
import { ref } from 'vue'

const isLoggedIn = ref(false) // TODO: 后续接入真实登录状态
const userName = ref('未登录')
const userEmail = ref('')
const membershipLevel = ref('基础版')

const goToLogin = () => {
  window.open('https://linjhs.com/login', '_blank')
}

const goToShop = () => {
  window.open('https://linjhs.com/shop/products', '_blank')
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
  <div class="profile-container min-h-screen bg-gray-50 py-8">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- User Card -->
      <div class="bg-white rounded-2xl shadow-lg overflow-hidden mb-8">
        <div class="bg-gradient-to-r from-purple-600 to-indigo-600 h-32"></div>
        <div class="px-8 pb-8">
          <div class="flex items-end -mt-16 mb-6">
            <div class="w-32 h-32 rounded-full bg-white shadow-lg flex items-center justify-center text-5xl border-4 border-white">
              {{ isLoggedIn ? '👤' : '🔒' }}
            </div>
          </div>
          
          <div v-if="!isLoggedIn" class="text-center py-8">
            <h2 class="text-2xl font-bold text-gray-900 mb-4">您还未登录</h2>
            <p class="text-gray-600 mb-6">登录后可以查看会员信息、转换历史等更多功能</p>
            <div class="flex gap-4 justify-center">
              <button 
                @click="goToLogin"
                class="bg-purple-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-purple-700 transition-colors"
              >
                立即登录
              </button>
              <button 
                @click="() => window.open('https://linjhs.com/signup', '_blank')"
                class="bg-gray-200 text-gray-700 px-8 py-3 rounded-lg font-semibold hover:bg-gray-300 transition-colors"
              >
                注册账号
              </button>
            </div>
          </div>

          <div v-else>
            <h2 class="text-2xl font-bold text-gray-900 mb-2">{{ userName }}</h2>
            <p class="text-gray-600 mb-4">{{ userEmail }}</p>
            <div class="inline-flex items-center px-4 py-2 rounded-full bg-purple-100 text-purple-700 font-semibold">
              <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path>
              </svg>
              {{ membershipLevel }}
            </div>
          </div>
        </div>
      </div>

      <!-- Menu Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div 
          v-for="(item, index) in menuItems" 
          :key="index"
          class="bg-white rounded-xl shadow-md hover:shadow-xl transition-all cursor-pointer p-6 transform hover:-translate-y-1"
          @click="item.title === '会员中心' ? goToShop() : null"
        >
          <div class="flex items-start">
            <div class="text-4xl mr-4">{{ item.icon }}</div>
            <div class="flex-1">
              <h3 class="text-lg font-semibold text-gray-900 mb-1">
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
      <div v-if="!isLoggedIn" class="mt-8 bg-gradient-to-r from-purple-50 to-indigo-50 rounded-2xl p-8 border border-purple-100">
        <h3 class="text-xl font-bold text-gray-900 mb-4">快速链接</h3>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <a 
            href="https://linjhs.com/login" 
            target="_blank"
            class="flex items-center justify-center bg-white px-6 py-4 rounded-lg hover:shadow-md transition-all"
          >
            <span class="font-semibold text-gray-700">登录账号</span>
            <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
            </svg>
          </a>
          <a 
            href="https://linjhs.com/signup" 
            target="_blank"
            class="flex items-center justify-center bg-white px-6 py-4 rounded-lg hover:shadow-md transition-all"
          >
            <span class="font-semibold text-gray-700">注册账号</span>
            <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
            </svg>
          </a>
          <a 
            href="https://linjhs.com/shop/products" 
            target="_blank"
            class="flex items-center justify-center bg-white px-6 py-4 rounded-lg hover:shadow-md transition-all"
          >
            <span class="font-semibold text-gray-700">购买会员</span>
            <svg class="w-5 h-5 ml-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
            </svg>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
