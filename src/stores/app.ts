import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  const unlocked = ref(false)       // 整体解锁
  const vaultLocked = ref(false)    // 密码库单独锁定
  const apiLocked = ref(false)      // API Key 单独锁定

  function lockVault() { vaultLocked.value = true }
  function unlockVault() { vaultLocked.value = false }
  function lockApi() { apiLocked.value = true }
  function unlockApi() { apiLocked.value = false }

  return { unlocked, vaultLocked, apiLocked, lockVault, unlockVault, lockApi, unlockApi }
})
