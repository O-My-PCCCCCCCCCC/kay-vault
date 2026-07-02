import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { clearClipboard } from '../utils/clipboard'

export type ThemeName = 'red' | 'blue' | 'purple' | 'green' | 'orange' | 'pink'

export const useAppStore = defineStore('app', () => {
  const unlocked = ref(false)       // 整体解锁
  const vaultLocked = ref(false)    // 密码库单独锁定（UI 级）
  const apiLocked = ref(false)      // API Key 单独锁定（UI 级）
  const sessionId = ref<string | null>(null)  // 会话令牌
  const autoLockMinutes = ref(5)    // 自动锁定分钟（0=永不）

  // 主题
  const theme = ref<ThemeName>((localStorage.getItem('kayTheme') as ThemeName) || 'red')

  function setTheme(t: ThemeName) {
    theme.value = t
    localStorage.setItem('kayTheme', t)
    document.documentElement.setAttribute('data-theme', t)
  }

  function lockVault() { vaultLocked.value = true }
  function unlockVault() { vaultLocked.value = false }
  function lockApi() { apiLocked.value = true }
  function unlockApi() { apiLocked.value = false }

  /** 登录：传密码 → 拿 sessionId */
  async function login(password: string): Promise<string> {
    const sid = await invoke<string>('session_login', { password })
    sessionId.value = sid
    unlocked.value = true
    return sid
  }

  /** 完全锁定：清 session + 清剪贴板 */
  async function logout() {
    await clearClipboard()
    if (sessionId.value) {
      try { await invoke('session_lock', { sessionId: sessionId.value }) } catch { /* 忽略 */ }
      sessionId.value = null
    }
    unlocked.value = false
    vaultLocked.value = false
    apiLocked.value = false
  }

  return {
    unlocked, vaultLocked, apiLocked, sessionId, autoLockMinutes, theme,
    setTheme, lockVault, unlockVault, lockApi, unlockApi, login, logout,
  }
})
