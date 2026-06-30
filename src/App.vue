<template>
  <n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <n-notification-provider>
        <n-dialog-provider>
          <LockScreen v-if="!appStore.unlocked" />
          <div v-else class="app-layout">
            <n-layout has-sider>
              <n-layout-sider bordered width="200" class="sider">
                <div class="sider-header">🔑 KVault</div>
                <n-menu
                  v-model:value="activeKey"
                  :options="menuOptions"
                  @update:value="onMenuChange"
                />
              </n-layout-sider>
              <n-layout>
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
import { ref, h } from 'vue'
import { useRouter } from 'vue-router'
import { darkTheme, zhCN, dateZhCN, NIcon } from 'naive-ui'
import { LockClosed20Filled as LockIcon, Settings20Filled as SettingsIcon } from '@vicons/fluent'
import { useAppStore } from './stores/app'
import LockScreen from './components/LockScreen.vue'

const appStore = useAppStore()
const router = useRouter()
const activeKey = ref('vault')

const menuOptions = [
  { label: '我的密码', key: 'vault', icon: () => h(NIcon, null, { default: () => h(LockIcon) }) },
  { label: '设置', key: 'settings', icon: () => h(NIcon, null, { default: () => h(SettingsIcon) }) },
]

function onMenuChange(key: string) {
  router.push(key === 'vault' ? '/' : '/settings')
}
</script>

<style scoped>
.app-layout { height: 100vh; }
.sider { background: var(--bg-secondary) !important; }
.sider-header { padding: 20px 16px; font-size: 18px; font-weight: 700; color: var(--accent-red); }
</style>
