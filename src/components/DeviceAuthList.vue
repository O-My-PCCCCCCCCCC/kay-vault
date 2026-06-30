<template>
  <div class="auth-section">
    <h3>设备认证</h3>
    <p class="section-desc">
      认证后本机才能执行备份和还原操作
    </p>

    <n-alert v-if="!authorized" type="warning" :bordered="false" style="margin-bottom: 12px;">
      本机尚未认证，无法备份/还原
    </n-alert>
    <n-alert v-else type="success" :bordered="false" style="margin-bottom: 12px;">
      本机已认证，可以执行备份/还原
    </n-alert>

    <n-button
      v-if="!authorized"
      type="primary"
      @click="confirmAdd"
      :loading="adding"
    >
      + 添加本机认证
    </n-button>
    <n-button
      v-else
      type="error"
      quaternary
      @click="confirmRemove"
    >
      删除本机认证
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useDialog, useMessage } from 'naive-ui'

const authorized = ref(false)
const adding = ref(false)
const dialog = useDialog()
const message = useMessage()

async function refresh() {
  try {
    authorized.value = await invoke<boolean>('auth_check')
  } catch {
    authorized.value = false
  }
}

function confirmAdd() {
  dialog.warning({
    title: '添加本机认证',
    content: '将在 C:/LuSh-Password-Backup 创建认证密钥文件，确定继续吗？',
    positiveText: '确认添加',
    negativeText: '取消',
    onPositiveClick: async () => {
      adding.value = true
      try {
        await invoke('auth_generate_key')
        message.success('本机已认证')
        await refresh()
      } catch (e: any) {
        message.error(String(e))
      } finally {
        adding.value = false
      }
    },
  })
}

function confirmRemove() {
  dialog.warning({
    title: '删除设备认证',
    content: '删除后本机将无法再备份/还原，确定继续吗？',
    positiveText: '确认删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await invoke('auth_remove')
        message.success('认证已删除')
        await refresh()
      } catch (e: any) {
        message.error(String(e))
      }
    },
  })
}

onMounted(refresh)
</script>

<style scoped>
.auth-section { padding: 8px 0; }
.section-desc { color: var(--text-secondary); font-size: 13px; margin: 4px 0 16px; }
</style>
