<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const navigateTo = (path: string) => {
  router.push(path)
}
</script>

<template>
  <nav class="bg-gradient-to-r from-purple-600 to-indigo-600 shadow-lg">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-14">
        <div class="flex items-center">
          <button @click="navigateTo('/')" class="flex items-center space-x-2">
            <svg class="w-7 h-7 text-white" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z"></path>
              <path fill-rule="evenodd" d="M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z" clip-rule="evenodd"></path>
            </svg>
            <span class="text-lg font-bold text-white">格式匠</span>
          </button>
        </div>
        
        <div class="flex items-center space-x-4">
          <button 
            @click="navigateTo('/')" 
            class="text-white hover:bg-white/10 px-2.5 py-1.5 rounded-md text-sm font-medium transition-colors"
          >
            首页
          </button>
          <button 
            @click="navigateTo('/upload')" 
            class="text-white hover:bg-white/10 px-2.5 py-1.5 rounded-md text-sm font-medium transition-colors"
          >
            转换文档
          </button>
          <button 
            @click="navigateTo('/profile')" 
            class="text-white hover:bg-white/10 px-2.5 py-1.5 rounded-md text-sm font-medium transition-colors"
          >
            我的
          </button>
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped>
</style>
