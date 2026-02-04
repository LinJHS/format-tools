<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

const links = {
  repo: 'https://github.com/LinJHS/format-tools',
  author1: 'https://linjhs.com',
  author2: 'https://monticule.tech',
  purchase: 'https://linjhs.com/shop/product/2',
  xhs: 'https://xhslink.com/m/51oKk1reGIi',
  bilibili: 'https://b23.tv/jHy9JyW',
  douyin: 'monticule.tech' 
}

const open = async (url: string) => {
    if (url) await openUrl(url)
}

const copyDouyin = async () => {
    try {
        await navigator.clipboard.writeText(links.douyin)
        alert('抖音号已复制: ' + links.douyin)
    } catch (e) {
        console.error(e)
        alert('复制失败，抖音号: ' + links.douyin)
    }
}
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 py-8">
    <div class="text-center mb-10">
        <div class="bg-linear-to-r from-purple-600 to-indigo-600 w-20 h-20 rounded-2xl mx-auto flex items-center justify-center shadow-lg mb-4">
            <span class="text-4xl text-white font-bold">匠</span>
        </div>
        <h1 class="text-3xl font-bold text-gray-800">格式匠 Format Tools</h1>
        <p class="text-gray-500 mt-2">版本 v0.1.0</p>
    </div>

    <div class="bg-white shadow rounded-lg overflow-hidden divide-y divide-gray-100">
        <!-- Project Info -->
        <div class="p-6">
            <h2 class="text-lg font-semibold text-gray-800 mb-4">项目信息</h2>
            <div class="space-y-3">
                <div class="flex items-center justify-between">
                    <span class="text-gray-600">开源仓库</span>
                    <button @click="open(links.repo)" class="text-blue-600 hover:underline">
                        GitHub: LinJHS/format-tools
                    </button>
                </div>
            </div>
        </div>

        <!-- Authors -->
        <div class="p-6">
            <h2 class="text-lg font-semibold text-gray-800 mb-4">作者 & 团队</h2>
            <div class="flex flex-col gap-4">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-gray-200 flex items-center justify-center text-gray-600">L</div>
                    <div>
                        <button @click="open(links.author1)" class="font-medium text-gray-800 hover:text-blue-600">@LinJHS</button>
                        <p class="text-xs text-gray-500">https://linjhs.com</p>
                    </div>
                </div>
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-indigo-100 flex items-center justify-center text-indigo-600">M</div>
                    <div>
                        <button @click="open(links.author2)" class="font-medium text-gray-800 hover:text-blue-600">@丘原科技</button>
                        <p class="text-xs text-gray-500">https://monticule.tech</p>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Social Media -->
        <div class="p-6">
             <h2 class="text-lg font-semibold text-gray-800 mb-4">关注我们</h2>
             <div class="grid grid-cols-2 sm:grid-cols-3 gap-4">
                 <button @click="open(links.xhs)" class="flex items-center gap-2 p-3 rounded-lg bg-red-50 hover:bg-red-100 transition text-red-700">
                     <span>📕</span> 小红书
                 </button>
                 <button @click="open(links.bilibili)" class="flex items-center gap-2 p-3 rounded-lg bg-pink-50 hover:bg-pink-100 transition text-pink-600">
                     <span>📺</span> Bilibili
                 </button>
                 <button @click="copyDouyin" class="flex items-center gap-2 p-3 rounded-lg bg-gray-50 hover:bg-gray-100 transition text-gray-700">
                     <span>🎵</span> 抖音 (点击复制)
                 </button>
             </div>
        </div>

        <!-- Purchase -->
        <div class="p-6 bg-yellow-50">
             <div class="flex justify-between items-center flex-wrap gap-4">
                 <div>
                     <h3 class="font-bold text-yellow-800">支持我们</h3>
                     <p class="text-sm text-yellow-700">解锁 专业版/大师版 高级功能</p>
                 </div>
                 <button @click="open(links.purchase)" class="bg-yellow-500 text-white px-6 py-2 rounded-full font-semibold shadow hover:bg-yellow-600 transition">
                     前往购买
                 </button>
             </div>
        </div>
    </div>
  </div>
</template>
