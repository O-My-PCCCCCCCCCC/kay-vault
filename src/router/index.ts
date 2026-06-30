import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  { path: '/', name: 'vault', component: () => import('../views/VaultView.vue') },
  { path: '/settings', name: 'settings', component: () => import('../views/SettingsView.vue') },
]

export default createRouter({
  history: createWebHistory(),
  routes,
})
