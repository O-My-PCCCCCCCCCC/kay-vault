<template>
  <div class="backup-section">
    <h3>备份与还原</h3>
    <p class="section-desc">备份目录: C:/LuSh-Password-Backup</p>

    <n-space vertical>
      <n-space>
        <n-button type="primary" :loading="backingUp" :disabled="!authorized" @click="doBackup">备份</n-button>
        <n-button type="warning" :loading="restoring" :disabled="!authorized" @click="confirmRestore">还原</n-button>
      </n-space>
      <n-button type="info" ghost @click="startImport" :loading="importing">导入备份文件</n-button>
    </n-space>

    <p v-if="!authorized" class="auth-warning">本机未认证，请先在「设备认证」中添加认证</p>
    <p v-if="lastBackup" class="backup-info">上次备份: {{ lastBackup }}</p>
    <p v-if="backupList.length > 0" class="backup-info">共有 {{ backupList.length }} 个备份文件</p>

    <!-- 导入密码弹窗 -->
    <n-modal v-model:show="showImportDialog" title="导入备份文件" preset="card" style="width:400px">
      <div class="import-body">
        <p class="import-path">{{ importFilePath }}</p>
        <n-input v-model:value="importPwd" type="password" size="large" placeholder="输入该备份文件的主密码" :disabled="importing" @keyup.enter="doImport" />
      </div>
      <div class="import-acts">
        <n-button size="small" @click="showImportDialog = false">取消</n-button>
        <n-button size="small" type="primary" :loading="importing" :disabled="!importPwd" @click="doImport">确认导入</n-button>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useMessage } from 'naive-ui'
import { useAppStore } from '../stores/app'

const appStore = useAppStore()
const authorized = ref(false)
const backingUp = ref(false)
const restoring = ref(false)
const importing = ref(false)
const lastBackup = ref('')
const backupList = ref<string[]>([])
const message = useMessage()

const showImportDialog = ref(false)
const importPwd = ref('')
const importFilePath = ref('')

async function refresh() {
  try {
    authorized.value = await invoke<boolean>('auth_check')
    backupList.value = await invoke<string[]>('list_backups')
  } catch { authorized.value = false }
}

async function doBackup() {
  backingUp.value = true
  try {
    const name = await invoke<string>('backup_now')
    lastBackup.value = name
    message.success('备份成功')
    await refresh()
  } catch (e: any) { message.error(String(e)) }
  finally { backingUp.value = false }
}

async function confirmRestore() {
  if (!window.confirm('还原将覆盖当前所有密码数据，确定继续？')) return
  restoring.value = true
  try {
    await invoke('restore_from_usb', { filename: null as any })
    message.success('还原成功')
    await appStore.logout()
  } catch (e: any) { message.error(String(e)) }
  finally { restoring.value = false }
}

async function startImport() {
  const selected = await open({
    multiple: false,
    filters: [{ name: '备份文件', extensions: ['enc'] }],
  })
  if (!selected) return
  importFilePath.value = selected as string
  importPwd.value = ''
  showImportDialog.value = true
}

async function doImport() {
  if (!importPwd.value || !importFilePath.value) return
  importing.value = true
  try {
    await invoke('import_from_file', {
      filePath: importFilePath.value,
      password: importPwd.value,
    })
    message.success('导入成功')
    showImportDialog.value = false
    await appStore.logout()
  } catch (e: any) { message.error(String(e)) }
  finally { importing.value = false }
}

onMounted(refresh)
</script>

<style scoped>
.backup-section { padding: 8px 0; }
.section-desc { color: var(--text-secondary); font-size: 13px; margin: 4px 0 16px; }
.auth-warning { color: var(--accent-red); font-size: 13px; margin-top: 8px; }
.backup-info { color: var(--text-muted); font-size: 12px; margin-top: 8px; }
.import-body { display: flex; flex-direction: column; gap: 12px; }
.import-path { font-size: 11px; color: var(--text-muted); word-break: break-all; font-family: monospace; background: var(--bg-secondary); padding: 6px 8px; border-radius: 4px; }
.import-acts { display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px; }
</style>
