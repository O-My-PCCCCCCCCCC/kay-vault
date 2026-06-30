<template>
  <div class="settings-view">
    <h2>设置</h2>

    <n-divider />

    <n-card title="安全" :bordered="false" class="setting-card">
      <n-form-item label="自动锁定时间">
        <n-select v-model:value="autoLock" :options="lockOptions" style="width: 160px" />
      </n-form-item>
      <n-button type="primary" @click="showChangePwd = true">修改主密码</n-button>
    </n-card>

    <n-divider />

    <n-card title="设备认证" :bordered="false" class="setting-card">
      <DeviceAuthList />
    </n-card>

    <n-divider />

    <n-card title="备份与还原" :bordered="false" class="setting-card">
      <BackupPanel />
    </n-card>

    <!-- 修改主密码弹窗 -->
    <n-modal v-model:show="showChangePwd" title="修改主密码" preset="card" style="width: 400px">
      <n-form label-placement="top">
        <n-form-item label="当前主密码">
          <n-input v-model:value="currentPwd" type="password" />
        </n-form-item>
        <n-form-item label="新主密码">
          <n-input v-model:value="newPwd" type="password" />
        </n-form-item>
        <n-form-item label="确认新密码">
          <n-input v-model:value="confirmPwd" type="password" />
        </n-form-item>
      </n-form>
      <div class="form-actions">
        <n-button @click="showChangePwd = false">取消</n-button>
        <n-button type="primary" @click="doChangePwd" :loading="changing">确认修改</n-button>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useMessage } from 'naive-ui'
import { useVaultStore } from '../stores/vault'
import DeviceAuthList from '../components/DeviceAuthList.vue'
import BackupPanel from '../components/BackupPanel.vue'

const vault = useVaultStore()
const message = useMessage()

const autoLock = ref(5)

const showChangePwd = ref(false)
const currentPwd = ref('')
const newPwd = ref('')
const confirmPwd = ref('')
const changing = ref(false)

const lockOptions = [
  { label: '1 分钟', value: 1 },
  { label: '5 分钟', value: 5 },
  { label: '15 分钟', value: 15 },
  { label: '30 分钟', value: 30 },
  { label: '永不', value: 0 },
]

async function doChangePwd() {
  if (!currentPwd.value || !newPwd.value) {
    message.warning('请填写完整')
    return
  }
  if (newPwd.value !== confirmPwd.value) {
    message.warning('两次输入的新密码不一致')
    return
  }
  if (currentPwd.value !== vault.masterPassword) {
    message.error('当前主密码错误')
    return
  }

  changing.value = true
  try {
    vault.masterPassword = newPwd.value
    await vault.saveToDisk()
    message.success('主密码已修改')
    showChangePwd.value = false
    currentPwd.value = ''
    newPwd.value = ''
    confirmPwd.value = ''
  } catch (e: any) {
    message.error(String(e))
  } finally {
    changing.value = false
  }
}
</script>

<style scoped>
.settings-view { padding: 24px; overflow-y: auto; height: 100vh; }
.setting-card { background: transparent; }
.form-actions { display: flex; justify-content: flex-end; gap: 12px; margin-top: 16px; }
</style>
