<template>
  <div class="settings-view">
    <h2 class="s-title">⚙️ 设置</h2>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">🔒 安全</div>
      <div class="s-row">
        <span class="s-label">自动锁定</span>
        <n-select v-model:value="autoLock" :options="lockOptions" style="width: 130px" size="small" />
      </div>
      <div class="s-row">
        <span class="s-label">主密码</span>
        <n-button size="small" type="primary" @click="showChangePwd = true">修改</n-button>
      </div>
    </div>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">🎨 主题</div>
      <div class="theme-list">
        <div v-for="t in themes" :key="t.id" class="theme-item" :class="{on:appStore.theme===t.id}" @click="onThemeChange(t.id)">
          <div class="ti-preview"><div class="ti-bar" :style="{background:t.accent}"></div><div class="ti-body" :style="{background:t.bg}"></div></div>
          <span class="ti-name">{{ t.label }}</span>
        </div>
      </div>
    </div>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">📂 备份路径</div>
      <div class="s-row s-row-col">
        <div class="s-path">{{ backupPath || '未设置' }}</div>
        <n-button size="small" @click="pickBackupFolder">选择文件夹</n-button>
      </div>
    </div>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">🔐 独立锁定</div>
      <div class="s-row">
        <span class="s-label">密码库</span>
        <n-button v-if="!appStore.vaultLocked" size="small" @click="doLockVault">🔒 锁定</n-button>
        <n-button v-else size="small" type="primary" @click="appStore.unlockVault();message.success('已解锁')">🔓 已锁定·解锁</n-button>
      </div>
      <div class="s-row">
        <span class="s-label">API 密钥</span>
        <n-button v-if="!appStore.apiLocked" size="small" @click="doLockApi">🔒 锁定</n-button>
        <n-button v-else size="small" type="primary" @click="appStore.unlockApi();message.success('已解锁')">🔓 已锁定·解锁</n-button>
      </div>
    </div>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">🔐 设备认证</div>
      <DeviceAuthList />
    </div>

    <n-divider style="margin: 8px 0" />

    <div class="s-group">
      <div class="s-group-title">💾 备份与还原</div>
      <BackupPanel />
    </div>

    <!-- 修改主密码弹窗 -->
    <n-modal v-model:show="showChangePwd" title="修改主密码" preset="card" style="width: 380px">
      <n-form label-placement="top">
        <n-form-item label="当前主密码">
          <n-input v-model:value="currentPwd" type="password" size="small" />
        </n-form-item>
        <n-form-item label="新主密码">
          <n-input v-model:value="newPwd" type="password" size="small" />
        </n-form-item>
        <n-form-item label="确认新密码">
          <n-input v-model:value="confirmPwd" type="password" size="small" />
        </n-form-item>
      </n-form>
      <div class="s-acts">
        <n-button size="small" @click="showChangePwd = false">取消</n-button>
        <n-button size="small" type="primary" @click="doChangePwd" :loading="changing">确认</n-button>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useAppStore } from '../stores/app'
import DeviceAuthList from '../components/DeviceAuthList.vue'
import BackupPanel from '../components/BackupPanel.vue'

const appStore = useAppStore()
const message = useMessage()
const autoLock = ref(5)
const backupPath = ref('')
const showChangePwd = ref(false)
const currentPwd = ref('')
const newPwd = ref('')
const confirmPwd = ref('')
const changing = ref(false)

const themes = [
  { id: 'red', label: '🔴 凯伊红', accent: '#E63946', bg: '#0a0e1a' },
  { id: 'blue', label: '🔵 深海蓝', accent: '#58a6ff', bg: '#0c1017' },
  { id: 'purple', label: '🟣 暗夜紫', accent: '#bc8cff', bg: '#0e0a18' },
  { id: 'green', label: '🟢 森林绿', accent: '#3fb950', bg: '#0a1410' },
  { id: 'orange', label: '🟠 落日橙', accent: '#d29922', bg: '#16100a' },
  { id: 'pink', label: '🩷 樱花粉', accent: '#ec8eb8', bg: '#160a10' },
]

const lockOptions = [
  { label: '1分', value: 1 }, { label: '5分', value: 5 },
  { label: '15分', value: 15 }, { label: '30分', value: 30 }, { label: '永不', value: 0 },
]

interface AppCfg { auto_lock_minutes: number; backup_path: string; categories: string[] }
const savedCfg = ref<AppCfg | null>(null)

// 加载配置
onMounted(async () => {
  try {
    const cfg = await invoke<AppCfg>('config_load')
    savedCfg.value = cfg
    autoLock.value = cfg.auto_lock_minutes ?? 5
    backupPath.value = cfg.backup_path || 'C:/LuSh-Password-Backup'
    appStore.autoLockMinutes = autoLock.value
  } catch { /* 使用默认值 */ }
})

// 保存配置（auto_lock_minutes + backup_path，保留 categories）
async function saveCfg() {
  if (!savedCfg.value) return
  try {
    await invoke('config_save', {
      cfg: {
        auto_lock_minutes: autoLock.value,
        backup_path: backupPath.value,
        categories: savedCfg.value.categories,
      },
    })
  } catch { /* 忽略 */ }
}

watch(autoLock, async (v) => {
  appStore.autoLockMinutes = v
  await saveCfg()
})

function onThemeChange(t: string) { appStore.setTheme(t as any) }

async function pickBackupFolder() {
  const selected = await open({ directory: true, multiple: false, title: '选择备份文件夹' })
  if (selected) {
    backupPath.value = selected as string
    await saveCfg()
    message.success('备份路径已更新')
  }
}

function doLockVault() {
  appStore.lockVault(); message.success('密码库已锁定')
}

function doLockApi() {
  appStore.lockApi(); message.success('API 密钥已锁定')
}

async function doChangePwd() {
  if (!currentPwd.value || !newPwd.value) { message.warning('请填写完整'); return }
  if (newPwd.value !== confirmPwd.value) { message.warning('两次密码不一致'); return }
  if (!appStore.sessionId) { message.error('未登录'); return }
  changing.value = true
  try {
    const newSessionId = await invoke<string>('session_change_password', {
      sessionId: appStore.sessionId,
      oldPassword: currentPwd.value,
      newPassword: newPwd.value,
    })
    appStore.sessionId = newSessionId
    message.success('已修改')
    showChangePwd.value = false
    currentPwd.value = ''; newPwd.value = ''; confirmPwd.value = ''
  } catch (e: any) { message.error(String(e)) }
  finally { changing.value = false }
}
</script>

<style scoped>
.settings-view { padding: 16px 20px; overflow-y: auto; height: 100vh; font-size: 13px; }
.s-title { font-size: 16px; font-weight: 700; color: var(--accent); }
.s-group { padding: 4px 0; }
.s-group-title { font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; margin-bottom: 8px; }
.s-row { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
.s-row-col { flex-direction: column; align-items: stretch; gap: 6px; }
.s-path { font-size: 12px; color: var(--text-secondary); font-family: monospace; padding: 6px 8px; background: rgba(255,255,255,0.03); border-radius: 4px; word-break: break-all; }
.s-label { font-size: 13px; color: var(--text-secondary); min-width: 80px; }
.s-acts { display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px; }
.theme-list { display: flex; flex-direction: column; gap: 4px; }
.theme-item { display: flex; align-items: center; gap: 10px; padding: 5px 8px; border-radius: var(--radius-xs); cursor: pointer; border: 1px solid var(--border); transition: all 0.12s; }
.theme-item:hover { border-color: var(--border-hover); background: var(--bg-hover); }
.theme-item.on { border-color: var(--accent); background: var(--accent-glow); }
.ti-preview { width: 32px; height: 20px; border-radius: 3px; overflow: hidden; display: flex; flex-direction: column; flex-shrink: 0; }
.ti-bar { height: 6px; }
.ti-body { flex: 1; }
.ti-name { font-size: 13px; color: var(--text-secondary); }
</style>
