import { createRouter, createWebHistory } from 'vue-router'
import { loadAuthRoutes, initAuthState } from '../auth/authLoader'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/upload',
    name: 'Upload',
    component: () => import('../views/Upload.vue')
  },
  {
    path: '/template',
    name: 'Template',
    component: () => import('../views/Template.vue')
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('../views/Profile.vue')
  }
  ,
  {
    path: '/result',
    name: 'Result',
    component: () => import('../views/Result.vue')
  },
  {
    path: '/history',
    name: 'History',
    component: () => import('../views/History.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Settings.vue')
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/About.vue')
  }
]


const router = createRouter({
  history: createWebHistory(),
  routes
})

// Dynamically load private auth routes when enabled and present
loadAuthRoutes(router)

// Initialize auth state from localStorage on app start
initAuthState()

export default router
