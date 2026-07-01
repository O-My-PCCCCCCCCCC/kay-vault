<template>
  <n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <n-notification-provider>
        <n-dialog-provider>
          <LockScreen v-if="!appStore.unlocked" />
          <div v-else class="app-layout">
            <n-layout has-sider style="height: 100%">
              <n-layout-sider bordered width="200" class="sider">
                <div class="sider-header">🔑 KVault</div>
                <n-menu
                  v-model:value="activeKey"
                  :options="menuOptions"
                  @update:value="onMenuChange"
                />
                <div class="sider-footer">
                  <div class="sf-row"><span>🔑 密码</span><span class="sf-val">{{ stats.password_count }}</span></div>
                  <div class="sf-row"><span>🔐 API</span><span class="sf-val">{{ stats.api_count }}</span></div>
                  <div class="sf-bar"><div class="sf-fill" :style="{ width: stats.disk_percent + '%' }"></div></div>
                  <div class="sf-row sf-small"><span>💾 {{ fmtSize(stats.disk_used) }} / {{ fmtSize(stats.disk_total) }}</span><span>{{ stats.disk_percent }}%</span></div>
                </div>
              </n-layout-sider>
              <n-layout style="height: 100%">
                <router-view />
              </n-layout>
            </n-layout>
          </div>
        </n-dialog-provider>
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from './stores/app'
import { useVaultStore } from './stores/vault'
import LockScreen from './components/LockScreen.vue'

const appStore = useAppStore()
const vaultStore = useVaultStore()
const router = useRouter()
const route = useRoute()
const activeKey = ref((route.name as string) || 'vault')

const stats = ref({
  password_count: 0,
  api_count: 0,
  disk_total: 0,
  disk_avail: 0,
  disk_used: 0,
  disk_percent: 0,
})

function fmtSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

async function refreshStats() {
  if (!vaultStore.masterPassword) return
  try {
    stats.value = await invoke('get_stats', { password: vaultStore.masterPassword })
  } catch {
    // 忽略，下次切换页面时再试
  }
}

watch(() => appStore.unlocked, (v) => { if (v) refreshStats() })
router.afterEach(() => refreshStats())

const menuOptions = [
  { label: '🔑 我的密码', key: 'vault' },
  { label: '🔐 API 密钥', key: 'api-keys' },
  { label: '🎲 PIN生成器', key: 'terminal' },
  { label: '⚙️ 设置', key: 'settings' },
]

const routeMap: Record<string, string> = {
  vault: '/', 'api-keys': '/api-keys', terminal: '/terminal', settings: '/settings',
}

function onMenuChange(key: string) {
  router.push(routeMap[key] || '/')
}
</script>

<style scoped>
.app-layout { height: 100vh; }
.sider {
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary) !important;
  border-right: 1px solid var(--border) !important;
}
.sider-header {
  padding: 18px 16px;
  font-size: 17px;
  font-weight: 700;
  color: var(--accent-red);
  letter-spacing: 0.5px;
  border-bottom: 1px solid var(--border);
}
.sider-footer {
  margin-top: auto;
  padding: 8px 12px;
  border-top: 1px solid var(--border);
  font-size: 11px;
}
.sf-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 2px 0;
  color: var(--text-muted);
}
.sf-val {
  color: var(--text-secondary);
  font-weight: 600;
  font-family: monospace;
}
.sf-small { font-size: 10px; }
.sf-bar {
  height: 3px;
  background: rgba(255, 255, 255, 0.06);
  border-radius: 2px;
  margin: 4px 0;
  overflow: hidden;
}
.sf-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--accent-red), #B31D28);
  border-radius: 2px;
  transition: width 0.3s;
}
</style>
