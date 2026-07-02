<template>
  <div class="password-row" @click="$emit('edit', entry)">
    <div class="row-left">
      <div class="site-icon">{{ entry.name.charAt(0).toUpperCase() }}</div>
      <div class="row-info">
        <div class="row-name">{{ entry.name }}</div>
        <div class="row-url">{{ entry.url || entry.username }}</div>
      </div>
    </div>
    <div class="row-creds">
      <div class="cred-item">
        <span class="cred-label">账号</span>
        <span class="cred-value">{{ entry.username }}</span>
        <n-button quaternary circle size="tiny" @click.stop="copyText(entry.username)">
          <template #icon><n-icon size="14"><CopyIcon /></n-icon></template>
        </n-button>
      </div>
      <div class="cred-item">
        <span class="cred-label">密码</span>
        <span class="cred-value">{{ showPassword ? entry.password : '••••••••••••' }}</span>
        <n-button quaternary circle size="tiny" @click.stop="showPassword = !showPassword">
          <template #icon><n-icon size="14">{{ showPassword ? EyeOffIcon : EyeIcon }}</n-icon></template>
        </n-button>
        <n-button quaternary circle size="tiny" @click.stop="copyText(entry.password)">
          <template #icon><n-icon size="14"><CopyIcon /></n-icon></template>
        </n-button>
      </div>
    </div>
    <div class="row-right">
      <n-tag size="tiny" :bordered="false" class="cat-tag">{{ entry.category }}</n-tag>
      <n-button-group size="tiny">
        <n-button quaternary @click.stop="$emit('edit', entry)">
          <template #icon><n-icon size="14"><EditIcon /></n-icon></template>
        </n-button>
        <n-button quaternary type="error" @click.stop="$emit('delete', entry)">
          <template #icon><n-icon size="14"><DeleteIcon /></n-icon></template>
        </n-button>
      </n-button-group>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { copySecure } from "../utils/clipboard"
import { useMessage } from 'naive-ui'
import {
  Copy20Filled as CopyIcon,
  Eye20Filled as EyeIcon,
  EyeOff20Filled as EyeOffIcon,
  Edit20Filled as EditIcon,
  Delete20Filled as DeleteIcon,
} from '@vicons/fluent'
import type { VaultEntry } from '../stores/vault'

defineProps<{ entry: VaultEntry }>()
defineEmits<{ edit: [entry: VaultEntry]; delete: [entry: VaultEntry] }>()

const showPassword = ref(false)
const message = useMessage()

async function copyText(text: string) {
  try {
    await copySecure(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}
</script>

<style scoped>
.password-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 16px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s;
}
.password-row:hover {
  border-color: var(--accent-red-glow);
  background: rgba(220, 38, 38, 0.03);
}
.password-row:not(:last-child) {
  border-bottom: none;
  border-radius: 0;
}
.password-row:first-child {
  border-radius: var(--radius-sm) var(--radius-sm) 0 0;
}
.password-row:last-child {
  border-radius: 0 0 var(--radius-sm) var(--radius-sm);
}
.password-row:only-child {
  border-radius: var(--radius-sm);
}

.row-left {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 160px;
  flex-shrink: 0;
}
.site-icon {
  width: 38px;
  height: 38px;
  border-radius: 10px;
  background: linear-gradient(135deg, var(--accent-red), var(--accent-red-soft));
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 16px;
  color: #fff;
  flex-shrink: 0;
}
.row-info {
  min-width: 0;
}
.row-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}
.row-url {
  font-size: 12px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-creds {
  flex: 1;
  display: flex;
  gap: 24px;
  min-width: 0;
}
.cred-item {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
  flex: 1;
}
.cred-label {
  color: var(--text-muted);
  font-size: 11px;
  flex-shrink: 0;
}
.cred-value {
  font-family: monospace;
  font-size: 13px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-right {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-shrink: 0;
}
.cat-tag {
  background: rgba(126, 200, 227, 0.1) !important;
  color: var(--accent-blue) !important;
}
</style>
