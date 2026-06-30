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
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
import { useAppStore } from './stores/app'
import LockScreen from './components/LockScreen.vue'

const appStore = useAppStore()
const router = useRouter()
const route = useRoute()
const activeKey = ref((route.name as string) || 'vault')

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
</style>
