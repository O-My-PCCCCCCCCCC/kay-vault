import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type ThemeName = 'red' | 'blue' | 'purple' | 'green' | 'orange' | 'pink' | 'custom'

export interface CustomTheme {
  accent: string
  bgPrimary: string
  bgCard: string
}

export const useAppStore = defineStore('app', () => {
  const unlocked = ref(false)       // 整体解锁
  const vaultLocked = ref(false)    // 密码库单独锁定（UI 级）
  const apiLocked = ref(false)      // API Key 单独锁定（UI 级）
  const sessionId = ref<string | null>(null)  // 会话令牌
  const autoLockMinutes = ref(5)    // 自动锁定分钟（0=永不）

  // 主题
  const theme = ref<ThemeName>((localStorage.getItem('kayTheme') as ThemeName) || 'red')
  const customColors = ref<CustomTheme>(JSON.parse(localStorage.getItem('kayCustomTheme') || 'null') || {
    accent: '#58a6ff',
    bgPrimary: '#0d1117',
    bgCard: '#1c2333',
  })

  function applyCustomTheme() {
    const el = document.documentElement
    el.setAttribute('data-theme', 'custom')
    const c = customColors.value
    el.style.setProperty('--accent', c.accent)
    el.style.setProperty('--accent-soft', c.accent)
    el.style.setProperty('--accent-glow', c.accent + '1f')
    el.style.setProperty('--accent-glow-strong', c.accent + '38')
    el.style.setProperty('--border-accent', c.accent + '4d')
    el.style.setProperty('--bg-primary', c.bgPrimary)
    el.style.setProperty('--bg-secondary', c.bgCard)
    el.style.setProperty('--bg-card', c.bgCard)
    el.style.setProperty('--bg-input', c.bgPrimary)
    el.style.setProperty('--bg-elevated', c.bgCard)
  }

  function setTheme(t: ThemeName) {
    theme.value = t
    localStorage.setItem('kayTheme', t)
    if (t === 'custom') {
      applyCustomTheme()
    } else {
      document.documentElement.removeAttribute('style')
      document.documentElement.setAttribute('data-theme', t)
    }
  }

  function updateCustomColors(colors: CustomTheme) {
    customColors.value = colors
    localStorage.setItem('kayCustomTheme', JSON.stringify(colors))
    if (theme.value === 'custom') applyCustomTheme()
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

  /** 完全锁定：清 session */
  async function logout() {
    if (sessionId.value) {
      try { await invoke('session_lock', { sessionId: sessionId.value }) } catch { /* 忽略 */ }
      sessionId.value = null
    }
    unlocked.value = false
    vaultLocked.value = false
    apiLocked.value = false
  }

  return {
    unlocked, vaultLocked, apiLocked, sessionId, autoLockMinutes, theme, customColors,
    setTheme, updateCustomColors, lockVault, unlockVault, lockApi, unlockApi, login, logout,
  }
})
