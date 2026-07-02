<template>
  <div class="term">
    <div class="term-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">➜  ~/settings</span></div>

    <div class="term-body">
      <div class="section">
        <div class="sec-title">┌──( <span class="sc">🔒 安全</span> )</div>

        <div class="row">
          <span class="rk">$</span> <span class="rc">auto-lock</span> <span class="rf">--分钟</span>
          <n-select v-model:value="autoLock" :options="lockOptions" style="width:120px" size="tiny" class="tsel" />
        </div>

        <div class="row">
          <span class="rk">$</span> <span class="rc">passwd</span> <span class="rf">--修改</span>
          <button class="btn" @click="showChangePwd = true">改密码</button>
        </div>
      </div>

      <div class="sec-div">───</div>

      <div class="section">
        <div class="sec-title">┌──( <span class="sc">🔐 独立锁定</span> )</div>

        <div class="row">
          <span class="rk">$</span> <span class="rc">vault</span>
          <button v-if="!appStore.vaultLocked" class="btn" @click="appStore.lockVault();msg.success('已锁定')">🔒 锁定</button>
          <button v-else class="btn suc" @click="appStore.unlockVault();msg.success('已解锁')">🔓 解锁</button>
        </div>

        <div class="row">
          <span class="rk">$</span> <span class="rc">api-keys</span>
          <button v-if="!appStore.apiLocked" class="btn" @click="appStore.lockApi();msg.success('已锁定')">🔒 锁定</button>
          <button v-else class="btn suc" @click="appStore.unlockApi();msg.success('已解锁')">🔓 解锁</button>
        </div>
      </div>

      <div class="sec-div">───</div>

      <div class="section">
        <div class="sec-title">┌──( <span class="sc">📂 备份路径</span> )</div>

        <div class="row row-col">
          <div class="path">{{ backupPath || '未设置' }}</div>
          <span class="rk">$</span> <span class="rc">backup</span> <span class="rf">--set-path</span>
          <button class="btn" @click="pickBackupFolder">选择文件夹</button>
        </div>
      </div>

      <div class="sec-div">───</div>

      <div class="section">
        <div class="sec-title">┌──( <span class="sc">🔐 设备认证</span> )</div>
        <DeviceAuthList />
      </div>

      <div class="sec-div">───</div>

      <div class="section">
        <div class="sec-title">┌──( <span class="sc">💾 备份与还原</span> )</div>
        <BackupPanel />
      </div>
    </div>

    <!-- 改密码弹窗 -->
    <div v-if="showChangePwd" class="modal-overlay" @click.self="showChangePwd = false">
      <div class="modal">
        <div class="modal-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">passwd --change</span></div>
        <div class="modal-body">
          <div class="row"><span class="rk">└─$</span> <span class="rc">当前密码</span> <input v-model="currentPwd" type="password" class="ti" placeholder="········" /></div>
          <div class="row"><span class="rk">└─$</span> <span class="rc">新密码</span> <input v-model="newPwd" type="password" class="ti" placeholder="········" /></div>
          <div class="row"><span class="rk">└─$</span> <span class="rc">确认密码</span> <input v-model="confirmPwd" type="password" class="ti" placeholder="········" /></div>
          <div class="row" style="margin-top:10px">
            <button class="btn" @click="showChangePwd = false">取消</button>
            <button class="btn suc" @click="doChangePwd" :disabled="changing">{{ changing ? '⏳' : '✓' }} 确认修改</button>
          </div>
        </div>
      </div>
    </div>
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
const msg = useMessage()
const autoLock = ref(5)
const backupPath = ref('')
const showChangePwd = ref(false)
const currentPwd = ref('')
const newPwd = ref('')
const confirmPwd = ref('')
const changing = ref(false)

const lockOptions = [
  { label: '1m', value: 1 }, { label: '5m', value: 5 },
  { label: '15m', value: 15 }, { label: '30m', value: 30 }, { label: '∞', value: 0 },
]

interface AppCfg { auto_lock_minutes: number; backup_path: string; categories: string[] }
const savedCfg = ref<AppCfg | null>(null)

onMounted(async () => {
  try {
    const cfg = await invoke<AppCfg>('config_load')
    savedCfg.value = cfg
    autoLock.value = cfg.auto_lock_minutes ?? 5
    backupPath.value = cfg.backup_path || 'C:/LuSh-Password-Backup'
    appStore.autoLockMinutes = autoLock.value
  } catch { /* 默认值 */ }
})

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
    msg.success('备份路径已更新')
  }
}

async function doChangePwd() {
  if (!currentPwd.value || !newPwd.value) { msg.warning('请填写完整'); return }
  if (newPwd.value !== confirmPwd.value) { msg.warning('两次密码不一致'); return }
  if (!appStore.sessionId) { msg.error('未登录'); return }
  changing.value = true
  try {
    const newSessionId = await invoke<string>('session_change_password', {
      sessionId: appStore.sessionId,
      oldPassword: currentPwd.value,
      newPassword: newPwd.value,
    })
    appStore.sessionId = newSessionId
    msg.success('已修改')
    showChangePwd.value = false
    currentPwd.value = ''; newPwd.value = ''; confirmPwd.value = ''
  } catch (e: any) { msg.error(String(e)) }
  finally { changing.value = false }
}
</script>

<style scoped>
.term { height: 100vh; display: flex; flex-direction: column; background: #0b0e14; font-family: 'Consolas','Courier New',monospace; font-size: 14px; color: #abb2bf; }

/* 标题栏 */
.term-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; flex-shrink: 0; }
.dot { width: 12px; height: 12px; border-radius: 50%; }
.red { background: #ff5f57; } .yellow { background: #ffbd2e; } .green { background: #28c840; }
.bar-text { margin-left: 8px; font-size: 12px; color: #5c6370; }

/* 内容 */
.term-body { flex: 1; overflow-y: auto; padding: 12px 20px 30px; }
.section { margin-bottom: 4px; }
.sec-title { color: #5c6370; font-size: 12px; margin-bottom: 8px; }
.sc { color: #e5c07b; }
.sec-div { color: #2c313a; font-size: 13px; margin: 8px 0; text-align: center; }

/* 行 */
.row { display: flex; align-items: center; gap: 6px; margin-bottom: 5px; flex-wrap: wrap; }
.row-col { flex-direction: column; align-items: stretch; gap: 4px; }
.rk { color: #98c379; font-size: 13px; flex-shrink: 0; }
.rc { color: #61afef; font-size: 13px; }
.rf { color: #d19a66; font-size: 12px; }

/* 输入框 */
.ti { background: transparent; border: none; outline: none; color: #abb2bf; font-family: inherit; font-size: 14px; width: 200px; border-bottom: 1px solid #2c313a; padding: 1px 4px; }
.ti:focus { border-bottom-color: #61afef; }
.ti::placeholder { color: #3b4048; }

/* 选择器 */
:deep(.tsel) .n-base-selection { background: #1a1d27 !important; border-color: #2c313a !important; }
:deep(.tsel) .n-base-selection-label { color: #abb2bf !important; font-family: inherit !important; font-size: 12px !important; }
:deep(.tsel) .n-base-selection-input { color: #abb2bf !important; }

/* 按钮 */
.btn { background: #2c313a; color: #abb2bf; border: none; padding: 4px 14px; border-radius: 4px; font-family: inherit; font-size: 12px; cursor: pointer; }
.btn:hover { background: #3b4048; color: #e5c07b; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }
.btn.suc { color: #98c379; }
.btn.suc:hover { color: #e5c07b; }

/* 路径显示 */
.path { font-size: 12px; color: #5c6370; padding: 4px 8px; background: #1a1d27; border-radius: 4px; word-break: break-all; font-family: inherit; }

/* 弹窗 */
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.6); display: flex; align-items: center; justify-content: center; z-index: 100; }
.modal { width: 380px; background: #0b0e14; border: 1px solid #2c313a; border-radius: 8px; overflow: hidden; }
.modal-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; }
.modal-body { padding: 16px 20px; }
</style>
