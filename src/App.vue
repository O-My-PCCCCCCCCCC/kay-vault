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
                <div class="sider-menu-wrap">
                  <n-menu
                    v-model:value="activeKey"
                    :options="menuOptions"
                    @update:value="onMenuChange"
                  />
                </div>
                <div class="sider-footer">
                  <div class="sf-stacked-bar">
                    <div class="sf-seg seg-other" :style="{ width: pctOther() + '%' }" title="其他数据"></div>
                    <div class="sf-seg seg-app" :style="{ width: pctApp() + '%' }" title="软件数据"></div>
                    <div class="sf-seg seg-pwd" :style="{ width: pctPwd() + '%' }" title="密码数据"></div>
                    <div class="sf-seg seg-free" :style="{ width: pctFree() + '%' }" title="剩余空间"></div>
                  </div>
                  <div class="sf-legend">
                    <span class="sf-legend-item"><i class="dot dot-other"></i>其他 {{ fmtSize(stats.other_bytes) }}</span>
                    <span class="sf-legend-item"><i class="dot dot-app"></i>软件 {{ fmtSize(stats.app_data_bytes) }}</span>
                    <span class="sf-legend-item"><i class="dot dot-pwd"></i>密码 {{ fmtSize(stats.password_data_bytes) }}</span>
                  </div>
                  <div class="sf-row">
                    <span>💾 {{ fmtSize(stats.disk_used) }} / {{ fmtSize(stats.disk_total) }}</span>
                    <span>{{ stats.disk_percent }}%</span>
                  </div>
                  <div class="sf-row">
                    <span>🔑 {{ stats.password_count }} · 🔐 {{ stats.api_count }}</span>
                  </div>
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
import { ref, watch, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useAppStore } from './stores/app'
import LockScreen from './components/LockScreen.vue'

const appStore = useAppStore()
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
  password_data_bytes: 0,
  app_data_bytes: 0,
  other_bytes: 0,
})

function fmtSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

async function refreshStats() {
  if (!appStore.sessionId) return
  try {
    stats.value = await invoke('get_stats', { sessionId: appStore.sessionId })
  } catch {
    // 忽略，下次切换页面时再试
  }
}

function pct(part: number): number {
  return stats.value.disk_total > 0 ? (part / stats.value.disk_total * 100) : 0
}
function pctOther() { return pct(stats.value.other_bytes) }
function pctApp() { return pct(stats.value.app_data_bytes) }
function pctPwd() { return pct(stats.value.password_data_bytes) }
function pctFree() { return pct(stats.value.disk_avail) }

// 防截屏：解锁后保护窗口内容，锁定后解除
async function setScreenshotProtection(protect: boolean) {
  try { await getCurrentWindow().setContentProtected(protect) } catch { /* 忽略 */ }
}

watch(() => appStore.unlocked, async (v) => {
  await setScreenshotProtection(v)
  if (v) refreshStats()
})

router.afterEach(() => refreshStats())

onMounted(() => { if (appStore.unlocked) setScreenshotProtection(true) })

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
  height: 100%;
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
  flex-shrink: 0;
}
.sider-menu-wrap {
  flex: 1;
  overflow-y: auto;
}
.sider-footer {
  flex-shrink: 0;
  padding: 8px 12px;
  border-top: 1px solid var(--border);
  font-size: 11px;
}

.sf-stacked-bar {
  display: flex;
  height: 8px;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 6px;
  background: rgba(255, 255, 255, 0.04);
}
.sf-seg { transition: width 0.3s; }
.seg-other { background: #5a5a6a; }
.seg-app { background: #5b9bd5; }
.seg-pwd { background: #4caf50; }
.seg-free { background: transparent; }
.sf-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-bottom: 4px;
}
.sf-legend-item {
  display: inline-flex;
  align-items: center;
  gap: 3px;
  color: var(--text-muted);
  font-size: 10px;
  white-space: nowrap;
}
.dot {
  display: inline-block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
}
.dot-other { background: #5a5a6a; }
.dot-app { background: #5b9bd5; }
.dot-pwd { background: #4caf50; }
.sf-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 1px 0;
  color: var(--text-muted);
}
.sf-val {
  color: var(--text-secondary);
  font-weight: 600;
  font-family: monospace;
}
</style>
