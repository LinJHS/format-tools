<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isLoggedIn = ref(false) // TODO: 后续接入真实登录状态

const features = [
  {
    icon: '📄',
    title: 'Markdown 转换',
    description: '快速将 Markdown 文件转换为专业的 Word 文档'
  },
  {
    icon: '🎨',
    title: '丰富模板',
    description: '提供多种专业模板，满足不同场景需求'
  },
  {
    icon: '🤖',
    title: 'AI 格式修复',
    description: '智能修复格式问题，确保输出文档完美无瑕'
  },
  {
    icon: '⚡',
    title: '极速转换',
    description: '本地处理，无需上传，转换速度快人一步'
  }
]

const startConvert = () => {
  router.push('/upload')
}

const goToLogin = () => {
  window.open('https://linjhs.com/login', '_blank')
}

const goToRegister = () => {
  window.open('https://linjhs.com/signup', '_blank')
}
</script>

<template>
  <div class="home-container">
    <!-- Hero Section -->
    <section class="hero-section bg-gradient-to-br from-purple-600 via-indigo-600 to-blue-600 text-white py-20">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center">
          <h1 class="text-5xl md:text-6xl font-bold mb-6 animate-fade-in">
            格式匠
          </h1>
          <p class="text-xl md:text-2xl mb-8 text-white/90">
            专业的 Markdown 到 Word 转换工具
          </p>
          <p class="text-lg mb-12 text-white/80 max-w-2xl mx-auto">
            无需复杂配置，一键转换。支持多种模板，让您的文档更加专业美观
          </p>
          
          <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <button 
              @click="startConvert"
              class="bg-white text-purple-600 px-8 py-4 rounded-lg text-lg font-semibold hover:bg-gray-100 transition-all transform hover:scale-105 shadow-lg"
            >
              开始转换 →
            </button>
            
            <div v-if="!isLoggedIn" class="flex gap-3">
              <button 
                @click="goToLogin"
                class="bg-transparent border-2 border-white text-white px-6 py-4 rounded-lg text-lg font-semibold hover:bg-white/10 transition-all"
              >
                登录
              </button>
              <button 
                @click="goToRegister"
                class="bg-transparent border-2 border-white text-white px-6 py-4 rounded-lg text-lg font-semibold hover:bg-white/10 transition-all"
              >
                注册
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Login Notice for Non-logged Users -->
    <section v-if="!isLoggedIn" class="bg-yellow-50 border-l-4 border-yellow-400 p-6 max-w-4xl mx-auto mt-8 rounded-r-lg">
      <div class="flex items-start">
        <div class="flex-shrink-0">
          <svg class="h-6 w-6 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm text-yellow-700">
            <strong class="font-semibold">登录后解锁更多功能！</strong>
            登录即可使用高级模板、AI 格式修复等专业功能。
            <a @click="goToLogin" class="underline cursor-pointer hover:text-yellow-900">立即登录</a>
          </p>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
            为什么选择格式匠？
          </h2>
          <p class="text-lg text-gray-600">
            强大的功能，简单的操作，让文档转换变得轻而易举
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          <div 
            v-for="(feature, index) in features" 
            :key="index"
            class="bg-gray-50 rounded-xl p-6 hover:shadow-xl transition-shadow cursor-pointer transform hover:-translate-y-1"
          >
            <div class="text-5xl mb-4">{{ feature.icon }}</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">
              {{ feature.title }}
            </h3>
            <p class="text-gray-600">
              {{ feature.description }}
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Membership Section -->
    <section class="py-20 bg-gradient-to-br from-gray-50 to-gray-100">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
            选择适合您的会员方案
          </h2>
          <p class="text-lg text-gray-600">
            从基础到专业，满足不同需求
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <!-- 基础版 -->
          <div class="bg-white rounded-2xl shadow-lg overflow-hidden">
            <div class="p-8">
              <h3 class="text-2xl font-bold text-gray-900 mb-2">基础版</h3>
              <p class="text-gray-600 mb-6">适合个人用户</p>
              <div class="mb-6">
                <span class="text-4xl font-bold text-gray-900">免费</span>
              </div>
              <ul class="space-y-3 mb-8">
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">基础 Markdown 转换</span>
                </li>
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">基础模板</span>
                </li>
              </ul>
              <button class="w-full bg-gray-200 text-gray-700 py-3 rounded-lg font-semibold hover:bg-gray-300 transition-colors">
                免费使用
              </button>
            </div>
          </div>

          <!-- 专业版 -->
          <div class="bg-white rounded-2xl shadow-xl overflow-hidden border-2 border-purple-600 transform scale-105">
            <div class="bg-purple-600 text-white text-center py-2 text-sm font-semibold">
              推荐
            </div>
            <div class="p-8">
              <h3 class="text-2xl font-bold text-gray-900 mb-2">专业版</h3>
              <p class="text-gray-600 mb-6">适合专业用户</p>
              <div class="mb-6">
                <span class="text-4xl font-bold text-gray-900">¥2.9</span>
                <span class="text-gray-600">/月</span>
              </div>
              <ul class="space-y-3 mb-8">
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">所有基础版功能</span>
                </li>
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">高级付费模板</span>
                </li>
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">优先技术支持</span>
                </li>
              </ul>
              <button 
                @click="() => window.open('https://linjhs.com/shop/products', '_blank')"
                class="w-full bg-purple-600 text-white py-3 rounded-lg font-semibold hover:bg-purple-700 transition-colors"
              >
                立即购买
              </button>
            </div>
          </div>

          <!-- 大师版 -->
          <div class="bg-white rounded-2xl shadow-lg overflow-hidden">
            <div class="p-8">
              <h3 class="text-2xl font-bold text-gray-900 mb-2">大师版</h3>
              <p class="text-gray-600 mb-6">适合团队和企业</p>
              <div class="mb-6">
                <span class="text-4xl font-bold text-gray-900">¥26.9</span>
                <span class="text-gray-600">/月</span>
              </div>
              <ul class="space-y-3 mb-8">
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">所有专业版功能</span>
                </li>
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">AI 格式修复</span>
                </li>
                <li class="flex items-start">
                  <svg class="w-5 h-5 text-green-500 mt-1 mr-2" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                  </svg>
                  <span class="text-gray-700">自定义模板生成</span>
                </li>
              </ul>
              <button 
                @click="() => window.open('https://linjhs.com/shop/products', '_blank')"
                class="w-full bg-indigo-600 text-white py-3 rounded-lg font-semibold hover:bg-indigo-700 transition-colors"
              >
                立即购买
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="py-20 bg-gradient-to-r from-purple-600 to-indigo-600 text-white">
      <div class="max-w-4xl mx-auto text-center px-4">
        <h2 class="text-3xl md:text-4xl font-bold mb-6">
          准备好开始了吗？
        </h2>
        <p class="text-xl mb-8 text-white/90">
          立即体验高效的文档转换服务
        </p>
        <button 
          @click="startConvert"
          class="bg-white text-purple-600 px-8 py-4 rounded-lg text-lg font-semibold hover:bg-gray-100 transition-all transform hover:scale-105 shadow-lg"
        >
          免费开始使用
        </button>
      </div>
    </section>
  </div>
</template>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 1s ease-out;
}
</style>
