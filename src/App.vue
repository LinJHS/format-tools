<script setup lang="ts">
import { onMounted } from 'vue'
import Navbar from './components/Navbar.vue'
import { checkForAppUpdate } from './composables/useUpdate'
import { useContextMenu } from './composables/useContextMenu'

useContextMenu()

onMounted(() => {
  // Silent check on startup
  checkForAppUpdate(true)
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 text-sm md:text-base">
    <Navbar />
    <router-view />
  </div>
</template>

<style>
body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>