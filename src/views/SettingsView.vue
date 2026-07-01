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
        <n-button v-else size="small" type="primary" @click="showUnlockVault = true">🔓 已锁定·解锁</n-button>
      </div>
      <div class="s-row">
        <span class="s-label">API 密钥</span>
        <n-button v-if="!appStore.apiLocked" size="small" @click="doLockApi">🔒 锁定</n-button>
        <n-button v-else size="small" type="primary" @click="showUnlockApi = true">🔓 已锁定·解锁</n-button>
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

    <!-- 解锁密码库 -->
    <n-modal v-model:show="showUnlockVault" title="解锁密码库" preset="card" style="width: 340px">
      <n-input v-model:value="unlockPwd" type="password" size="large" placeholder="输入主密码解锁" @keyup.enter="doUnlockVault" />
      <div class="s-acts">
        <n-button size="small" @click="showUnlockVault = false">取消</n-button>
        <n-button size="small" type="primary" @click="doUnlockVault">解锁</n-button>
      </div>
    </n-modal>

    <!-- 解锁 API -->
    <n-modal v-model:show="showUnlockApi" title="解锁 API 密钥" preset="card" style="width: 340px">
      <n-input v-model:value="unlockPwd" type="password" size="large" placeholder="输入主密码解锁" @keyup.enter="doUnlockApi" />
      <div class="s-acts">
        <n-button size="small" @click="showUnlockApi = false">取消</n-button>
        <n-button size="small" type="primary" @click="doUnlockApi">解锁</n-button>
      </div>
    </n-modal>

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
const showUnlockVault = ref(false)
const showUnlockApi = ref(false)
const unlockPwd = ref('')
const currentPwd = ref('')
const newPwd = ref('')
const confirmPwd = ref('')
const changing = ref(false)

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

function doUnlockVault() {
  appStore.unlockVault(); showUnlockVault.value = false
  message.success('密码库已解锁')
}

function doUnlockApi() {
  appStore.unlockApi(); showUnlockApi.value = false
  message.success('API 密钥已解锁')
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
.s-title { font-size: 16px; font-weight: 700; color: var(--accent-red); }
.s-group { padding: 4px 0; }
.s-group-title { font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1px; margin-bottom: 8px; }
.s-row { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
.s-row-col { flex-direction: column; align-items: stretch; gap: 6px; }
.s-path { font-size: 12px; color: var(--text-secondary); font-family: monospace; padding: 6px 8px; background: rgba(255,255,255,0.03); border-radius: 4px; word-break: break-all; }
.s-label { font-size: 13px; color: var(--text-secondary); min-width: 80px; }
.s-acts { display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px; }
</style>
