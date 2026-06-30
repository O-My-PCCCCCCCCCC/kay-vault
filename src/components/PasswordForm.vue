<template>
  <div class="form-container">
    <n-form :model="form" label-placement="top">
      <n-form-item label="名称">
        <n-input v-model:value="form.name" placeholder="例如：GitHub" />
      </n-form-item>
      <n-form-item label="URL">
        <n-input v-model:value="form.url" placeholder="github.com" />
      </n-form-item>
      <n-form-item label="账号">
        <n-input v-model:value="form.username" placeholder="用户名或邮箱" />
      </n-form-item>
      <n-form-item label="密码">
        <n-input
          v-model:value="form.password"
          :type="showPwd ? 'text' : 'password'"
          placeholder="输入密码或点击生成"
        >
          <template #suffix>
            <n-button @click="showPwd = !showPwd" size="tiny" quaternary>
              {{ showPwd ? '🙈' : '👁️' }}
            </n-button>
            <n-button @click="showGenerator = true" size="tiny" quaternary>
              🎲 生成
            </n-button>
          </template>
        </n-input>
      </n-form-item>
      <n-form-item label="分类">
        <n-select v-model:value="form.category" :options="categoryOptions" />
      </n-form-item>
      <n-form-item label="备注">
        <n-input v-model:value="form.notes" type="textarea" rows="3" />
      </n-form-item>
    </n-form>

    <div class="form-actions">
      <n-button @click="$emit('close')">取消</n-button>
      <n-button type="primary" @click="handleSave">保存</n-button>
    </div>

    <PasswordGenerator
      :visible="showGenerator"
      @select="onPasswordSelected"
      @close="showGenerator = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useVaultStore, type VaultEntry } from '../stores/vault'
import PasswordGenerator from './PasswordGenerator.vue'

const props = defineProps<{ entry: VaultEntry | null }>()
const emit = defineEmits<{ save: [entry: VaultEntry]; close: [] }>()

const vault = useVaultStore()
const showGenerator = ref(false)
const showPwd = ref(false)

const categoryOptions = computed(() =>
  vault.entries.length > 0
    ? [...new Set(vault.entries.map(e => e.category))].map(c => ({ label: c, value: c }))
    : [
        { label: '社交账号', value: '社交账号' },
        { label: '开发工具', value: '开发工具' },
        { label: '金融支付', value: '金融支付' },
        { label: '邮箱', value: '邮箱' },
        { label: '娱乐游戏', value: '娱乐游戏' },
        { label: '其他', value: '其他' },
      ]
)

const form = reactive<VaultEntry>({
  id: props.entry?.id || crypto.randomUUID(),
  name: props.entry?.name || '',
  url: props.entry?.url || '',
  username: props.entry?.username || '',
  password: props.entry?.password || '',
  notes: props.entry?.notes || '',
  category: props.entry?.category || '其他',
  created_at: props.entry?.created_at || new Date().toISOString(),
  updated_at: new Date().toISOString(),
})

function onPasswordSelected(pwd: string) {
  form.password = pwd
}

function handleSave() {
  if (!form.name.trim()) return
  form.updated_at = new Date().toISOString()
  emit('save', { ...form })
}
</script>

<style scoped>
.form-container { padding: 12px 0; }
.form-actions { display: flex; justify-content: flex-end; gap: 12px; margin-top: 16px; }
</style>
