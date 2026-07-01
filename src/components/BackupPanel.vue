<template>
  <div class="backup-section">
    <h3>备份与还原</h3>
    <p class="section-desc">备份目录: C:/LuSh-Password-Backup</p>

    <n-space vertical>
      <n-space>
        <n-button
          type="primary"
          :loading="backingUp"
          :disabled="!authorized"
          @click="doBackup"
        >
          备份到 C:/LuSh-Password-Backup
        </n-button>
        <n-button
          type="warning"
          :loading="restoring"
          :disabled="!authorized"
          @click="confirmRestore"
        >
          从最新备份还原
        </n-button>
      </n-space>

      <n-button
        type="info"
        ghost
        @click="importFile"
        :loading="importing"
      >
        导入备份文件（从本机选择 .enc 文件）
      </n-button>
    </n-space>

    <p v-if="!authorized" class="auth-warning">本机未认证，请先在「设备认证」中添加认证</p>
    <p v-if="lastBackup" class="backup-info">上次备份: {{ lastBackup }}</p>
    <p v-if="backupList.length > 0" class="backup-info">共有 {{ backupList.length }} 个备份文件</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useMessage } from 'naive-ui'

const authorized = ref(false)
const backingUp = ref(false)
const restoring = ref(false)
const importing = ref(false)
const lastBackup = ref('')
const backupList = ref<string[]>([])
const message = useMessage()

async function refresh() {
  try {
    authorized.value = await invoke<boolean>('auth_check')
    backupList.value = await invoke<string[]>('list_backups')
  } catch {
    authorized.value = false
  }
}

async function doBackup() {
  backingUp.value = true
  try {
    const name = await invoke<string>('backup_now')
    lastBackup.value = name
    message.success('备份成功')
    await refresh()
  } catch (e: any) {
    message.error(String(e))
  } finally {
    backingUp.value = false
  }
}

async function confirmRestore() {
  if (!window.confirm('还原将覆盖当前所有密码数据，确定继续？')) return
  restoring.value = true
  try {
    await invoke('restore_from_usb', { filename: null as any })
    message.success('还原成功，请重新解锁')
  } catch (e: any) {
    message.error(String(e))
  } finally {
    restoring.value = false
  }
}

async function importFile() {
  // 打开系统原生文件选择器，筛选 .enc 文件
  const selected = await open({
    multiple: false,
    filters: [{
      name: '备份文件',
      extensions: ['enc'],
    }],
  })
  if (!selected) return // 用户取消

  const filePath = selected as string

  // 需要输入该备份文件对应的主密码
  const importPwd = window.prompt(`请输入该备份文件的主密码：\n${filePath}`)
  if (!importPwd) return

  if (!window.confirm(`将导入 ${filePath} 并覆盖当前所有密码数据，确定继续？`)) return
  importing.value = true
  try {
    await invoke('import_from_file', {
      filePath,
      password: importPwd,
    })
    message.success('导入成功，请重新解锁')
  } catch (e: any) {
    message.error(String(e))
  } finally {
    importing.value = false
  }
}

onMounted(refresh)
</script>

<style scoped>
.backup-section { padding: 8px 0; }
.section-desc { color: var(--text-secondary); font-size: 13px; margin: 4px 0 16px; }
.auth-warning { color: var(--accent-red); font-size: 13px; margin-top: 8px; }
.backup-info { color: var(--text-muted); font-size: 12px; margin-top: 8px; }
</style>
