<template>
  <div class="lock-screen">
    <div class="lock-card">
      <div class="logo"><img src="/icons/app.png" class="lock-icon" /></div>
      <h2>凯伊密码管家</h2>
      <p class="subtitle">插入钥匙以继续</p>
      <div class="warning-banner">
        ⚠️ 请妥善保管主密码和备份文件<br>
        主密码丢失后加密数据无法找回
      </div>
      <n-input
        v-model:value="password"
        type="password"
        placeholder="输入主密码"
        size="large"
        :disabled="loading"
        :input-props="{ autocomplete: 'off', spellcheck: 'false', style: 'ime-mode: disabled' }"
        @keyup.enter="unlock"
      />
      <n-button
        type="primary"
        size="large"
        block
        style="margin-top: 16px"
        :loading="loading"
        @click="unlock"
      >
        用钥匙打开
      </n-button>
      <p v-if="errorMsg" class="error">{{ errorMsg }}</p>
      <p class="hint">首次使用？输入任意密码即可创建密码库</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAppStore } from '../stores/app'
import { useVaultStore } from '../stores/vault'

const appStore = useAppStore()
const vault = useVaultStore()

const password = ref('')
const loading = ref(false)
const errorMsg = ref('')

async function unlock() {
  if (!password.value) return
  loading.value = true
  errorMsg.value = ''
  try {
    await appStore.login(password.value)
    await vault.loadFromDisk()
  } catch (e: any) {
    errorMsg.value = String(e) || '主密码错误'
    appStore.unlocked = false
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.lock-screen {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
}
.lock-card {
  width: 360px;
  padding: 40px;
  background: var(--bg-secondary);
  border-radius: var(--radius);
  border: 1px solid var(--border);
  text-align: center;
}
.logo { margin-bottom: 16px; }
.lock-icon { width: 64px; height: 64px; border-radius: 16px; }
.subtitle {
  color: var(--text-secondary);
  margin: 8px 0 24px;
  font-size: 14px;
}
.warning-banner {
  background: var(--accent-glow);
  border: 1px solid var(--border-accent);
  border-radius: 6px;
  padding: 10px 14px;
  margin-bottom: 20px;
  font-size: 12px;
  color: var(--accent);
  line-height: 1.6;
  text-align: left;
}
.error {
  color: var(--accent);
  margin-top: 12px;
  font-size: 13px;
}
.hint {
  color: var(--text-muted);
  margin-top: 16px;
  font-size: 12px;
}
</style>
