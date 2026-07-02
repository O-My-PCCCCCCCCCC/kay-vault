import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  { path: '/', name: 'vault', component: () => import('../views/VaultView.vue') },
  { path: '/settings', name: 'settings', component: () => import('../views/SettingsView.vue') },
  { path: '/terminal', name: 'terminal', component: () => import('../views/TerminalView.vue') },
  { path: '/api-keys', name: 'api-keys', component: () => import('../views/ApiKeysView.vue') },
  { path: '/generator', name: 'generator', component: () => import('../views/GeneratorView.vue') },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})
