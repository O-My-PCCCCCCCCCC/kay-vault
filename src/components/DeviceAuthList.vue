<template>
  <div class="auth-section">
    <h3>已认证设备</h3>
    <p class="section-desc">插入 U 盘以管理设备认证</p>

    <n-alert v-if="!usbInserted" type="info" :bordered="false" style="margin-bottom: 12px;">
      请插入 U 盘以管理设备认证
    </n-alert>

    <template v-if="usbInserted">
      <n-list v-if="devices.length > 0" bordered>
        <n-list-item v-for="device in devices" :key="device">
          <div class="device-item">
            <span>{{ device }}</span>
            <n-button size="small" type="error" quaternary @click="confirmRemove(device)">
              删除认证
            </n-button>
          </div>
        </n-list-item>
      </n-list>
      <n-empty v-else description="暂无已认证设备" style="margin: 24px 0" />

      <n-button type="primary" @click="addDevice" :loading="adding" style="margin-top: 12px">
        + 添加本机认证
      </n-button>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDialog, useMessage } from 'naive-ui'

const props = defineProps<{ usbPath: string; usbInserted: boolean }>()
const devices = ref<string[]>([])
const adding = ref(false)
const dialog = useDialog()
const message = useMessage()

watch(() => props.usbInserted, async (v) => {
  if (v) await refresh()
})

async function refresh() {
  if (!props.usbInserted) return
  try {
    devices.value = await invoke<string[]>('auth_list_devices', { usbPath: props.usbPath })
  } catch { /* ignore */ }
}

async function addDevice() {
  adding.value = true
  try {
    const name = await invoke<string>('auth_generate_key', { usbPath: props.usbPath })
    message.success(`设备「${name}」已认证`)
    await refresh()
  } catch (e: any) {
    message.error(String(e))
  } finally {
    adding.value = false
  }
}

function confirmRemove(device: string) {
  dialog.warning({
    title: '删除设备认证',
    content: `确定要删除「${device}」的认证吗？删除后该设备将无法再备份/还原。`,
    positiveText: '确认删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await invoke('auth_remove', { usbPath: props.usbPath, deviceName: device })
        message.success('已删除')
        await refresh()
      } catch (e: any) {
        message.error(String(e))
      }
    },
  })
}
</script>

<style scoped>
.auth-section { padding: 8px 0; }
.section-desc { color: var(--text-secondary); font-size: 13px; margin: 4px 0 16px; }
.device-item { display: flex; justify-content: space-between; align-items: center; }
</style>
