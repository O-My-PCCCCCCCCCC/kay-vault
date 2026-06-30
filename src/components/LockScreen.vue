<template>
  <div class="lock-screen">
    <div class="lock-card">
      <div class="logo">🔑</div>
      <h2>凯伊密码管家</h2>
      <p class="subtitle">插入钥匙以继续</p>
      <n-input
        v-model:value="password"
        type="password"
        placeholder="输入主密码"
        size="large"
        :input-props="{ autofocus: true }"
        @keyup.enter="unlock"
      />
      <n-button type="primary" size="large" block style="margin-top: 16px;" @click="unlock">
        用钥匙打开
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAppStore } from '../stores/app'

const appStore = useAppStore()
const password = ref('')

function unlock() {
  if (password.value) {
    appStore.unlocked = true
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
.logo {
  font-size: 48px;
  margin-bottom: 16px;
}
.subtitle {
  color: var(--text-secondary);
  margin: 8px 0 24px;
  font-size: 14px;
}
</style>
