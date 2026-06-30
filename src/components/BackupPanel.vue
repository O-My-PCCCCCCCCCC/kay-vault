<template>
  <div class="backup-section">
    <h3>备份与还原</h3>
    <p class="section-desc">将密码库备份到 U 盘，或从 U 盘还原</p>

    <n-alert v-if="!usbInserted" type="info" :bordered="false" style="margin-bottom: 12px;">
      请插入 U 盘以使用备份/还原功能
    </n-alert>

    <template v-if="usbInserted">
      <n-space>
        <n-button
          type="primary"
          :loading="backingUp"
          :disabled="!authorized"
          @click="doBackup"
        >
          备份到 U 盘
        </n-button>
        <n-button
          type="warning"
          :loading="restoring"
          :disabled="!authorized"
          @click="confirmRestore"
        >
          从 U 盘还原
        </n-button>
      </n-space>
      <p v-if="!authorized" class="auth-warning">本机未认证，请先在「设备认证」中添加认证</p>
      <p v-if="lastBackup" class="backup-info">上次备份: {{ lastBackup }}</p>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDialog, useMessage } from 'naive-ui'

const props = defineProps<{ usbPath: string; usbInserted: boolean }>()
const authorized = ref(false)
const backingUp = ref(false)
const restoring = ref(false)
const lastBackup = ref('')
const dialog = useDialog()
const message = useMessage()

watch(() => props.usbInserted, async (v) => {
  if (v) {
    try {
      authorized.value = await invoke<boolean>('auth_check', { usbPath: props.usbPath })
    } catch { authorized.value = false }
  } else {
    authorized.value = false
  }
})

async function doBackup() {
  backingUp.value = true
  try {
    const name = await invoke<string>('backup_now', { usbPath: props.usbPath })
    lastBackup.value = name
    message.success('备份成功')
  } catch (e: any) {
    message.error(String(e))
  } finally {
    backingUp.value = false
  }
}

function confirmRestore() {
  dialog.warning({
    title: '还原密码库',
    content: '还原将覆盖当前所有密码数据，确定继续？',
    positiveText: '确认还原',
    negativeText: '取消',
    onPositiveClick: async () => {
      restoring.value = true
      try {
        await invoke('restore_from_usb', { usbPath: props.usbPath, filename: null as any })
        message.success('还原成功，请重新解锁')
      } catch (e: any) {
        message.error(String(e))
      } finally {
        restoring.value = false
      }
    },
  })
}
</script>

<style scoped>
.backup-section { padding: 8px 0; }
.section-desc { color: var(--text-secondary); font-size: 13px; margin: 4px 0 16px; }
.auth-warning { color: var(--accent-red); font-size: 13px; margin-top: 8px; }
.backup-info { color: var(--text-muted); font-size: 12px; margin-top: 8px; }
</style>
