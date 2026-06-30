<template>
  <n-card class="password-card" :bordered="true" hoverable>
    <div class="card-header">
      <span class="card-name">{{ entry.name }}</span>
      <n-tag size="small" :bordered="false">{{ entry.category }}</n-tag>
    </div>
    <div class="card-body">
      <div class="field">
        <span class="label">账号</span>
        <span class="value">{{ entry.username }}</span>
        <n-button quaternary circle size="tiny" @click="copyText(entry.username)">
          <template #icon><n-icon><CopyIcon /></n-icon></template>
        </n-button>
      </div>
      <div class="field">
        <span class="label">密码</span>
        <span class="value">{{ showPassword ? entry.password : '••••••••' }}</span>
        <n-button quaternary circle size="tiny" @click="showPassword = !showPassword">
          <template #icon><n-icon>{{ showPassword ? EyeOffIcon : EyeIcon }}</n-icon></template>
        </n-button>
        <n-button quaternary circle size="tiny" @click="copyText(entry.password)">
          <template #icon><n-icon><CopyIcon /></n-icon></template>
        </n-button>
      </div>
    </div>
    <div class="card-footer">
      <span class="updated">更新于 {{ formatDate(entry.updated_at) }}</span>
      <div class="actions">
        <n-button size="tiny" quaternary @click="$emit('edit', entry)">编辑</n-button>
        <n-button size="tiny" quaternary type="error" @click="$emit('delete', entry)">删除</n-button>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useMessage } from 'naive-ui'
import { Copy20Filled as CopyIcon } from '@vicons/fluent'
import { Eye20Filled as EyeIcon, EyeOff20Filled as EyeOffIcon } from '@vicons/fluent'
import type { VaultEntry } from '../stores/vault'

const props = defineProps<{ entry: VaultEntry }>()
defineEmits<{ edit: [entry: VaultEntry]; delete: [entry: VaultEntry] }>()

const showPassword = ref(false)
const message = useMessage()

async function copyText(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    message.success('已复制')
  } catch {
    message.error('复制失败')
  }
}

function formatDate(iso: string): string {
  if (!iso) return ''
  return iso.slice(0, 10)
}
</script>

<style scoped>
.password-card { background: var(--bg-card); border-color: var(--border); }
.card-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.card-name { font-size: 16px; font-weight: 600; }
.field { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.label { color: var(--text-muted); font-size: 12px; min-width: 32px; }
.value { flex: 1; font-family: monospace; font-size: 14px; color: var(--text-primary); }
.card-footer { display: flex; justify-content: space-between; align-items: center; margin-top: 12px; padding-top: 10px; border-top: 1px solid var(--border); }
.updated { color: var(--text-muted); font-size: 12px; }
.actions { display: flex; gap: 4px; }
</style>
