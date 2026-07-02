<template>
  <div class="form-wrap">
    <!-- 顶部标题装饰 -->
    <div class="form-head">
      <span class="head-icon">{{ editingEntry ? '✏️' : '➕' }}</span>
      <span>{{ editingEntry ? '编辑密码' : '新建密码' }}</span>
    </div>

    <div class="form-body">
      <!-- 名称 + URL 同行 -->
      <div class="f-row">
        <div class="f-group flex-1">
          <label class="f-label">📛 名称</label>
          <n-input v-model:value="form.name" placeholder="例如：GitHub" size="large" />
        </div>
        <div class="f-group flex-1">
          <label class="f-label">🔗 URL</label>
          <n-input v-model:value="form.url" placeholder="github.com" size="large" />
        </div>
      </div>

      <!-- 账号 -->
      <div class="f-group">
        <label class="f-label">👤 账号</label>
        <n-input v-model:value="form.username" placeholder="用户名或邮箱" size="large" />
      </div>

      <!-- 密码 — 带生成器按钮 -->
      <div class="f-group">
        <label class="f-label">🔑 密码</label>
        <n-input
          v-model:value="form.password"
          :type="showPwd ? 'text' : 'password'"
          placeholder="输入密码或点击生成"
          size="large"
        >
          <template #suffix>
            <span class="pwd-act" @click="showPwd = !showPwd" :title="showPwd ? '隐藏' : '显示'">
              {{ showPwd ? '🙈' : '👁️' }}
            </span>
            <span class="pwd-act gen" @click="showGenerator = true" title="生成随机密码">🎲</span>
          </template>
        </n-input>
      </div>

      <!-- 分组 + 分类 同行 -->
      <div class="f-row">
        <div class="f-group flex-1">
          <label class="f-label">📦 分组</label>
          <n-auto-complete
            v-model:value="form.group"
            :options="existingGroups"
            placeholder="输入或选择"
            clearable
            size="large"
          />
        </div>
        <div class="f-group flex-1">
          <label class="f-label">📁 分类</label>
          <n-select v-model:value="form.category" :options="categoryOptions" tag clearable size="large" />
        </div>
      </div>

      <!-- 备注 -->
      <div class="f-group">
        <label class="f-label">📝 备注</label>
        <n-input v-model:value="form.notes" type="textarea" :rows="2" placeholder="选填" />
      </div>
    </div>

    <!-- 底部按钮 -->
    <div class="form-foot">
      <n-button size="large" ghost @click="$emit('close')" class="btn-cancel">取消</n-button>
      <n-button size="large" type="primary" @click="handleSave" class="btn-save">
        {{ editingEntry ? '✏️ 保存修改' : '➕ 创建密码' }}
      </n-button>
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

const existingGroups = computed(() => {
  const defaults = [
    { label: '默认分组', value: '默认分组' },
    { label: '💼 工作', value: '工作' },
    { label: '👤 个人', value: '个人' },
    { label: '👥 社交', value: '社交' },
    { label: '💰 金融', value: '金融' },
    { label: '💻 开发', value: '开发' },
    { label: '🎮 娱乐', value: '娱乐' },
    { label: '🛒 购物', value: '购物' },
    { label: '📚 教育', value: '教育' },
    { label: '🏥 医疗', value: '医疗' },
    { label: '📦 其他', value: '其他' },
  ]
  const extra = [...new Set(vault.entries.map(e => e.group).filter(Boolean))]
    .filter(g => !defaults.some(d => d.value === g))
    .map(g => ({ label: g, value: g }))
  return [...defaults, ...extra]
})

const categoryOptions = computed(() => {
  const cats = new Set(vault.entries.map(e => e.category).filter(Boolean))
  const defaults = [
    { label: '社交账号', value: '社交账号' },
    { label: '开发工具', value: '开发工具' },
    { label: '金融支付', value: '金融支付' },
    { label: '邮箱', value: '邮箱' },
    { label: '娱乐游戏', value: '娱乐游戏' },
    { label: '其他', value: '其他' },
  ]
  const extra = Array.from(cats)
    .filter(c => !defaults.some(d => d.value === c))
    .map(c => ({ label: c, value: c }))
  return [...defaults, ...extra]
})

const editingEntry = computed(() => props.entry)

const form = reactive<VaultEntry>({
  id: props.entry?.id || crypto.randomUUID(),
  name: props.entry?.name || '',
  url: props.entry?.url || '',
  username: props.entry?.username || '',
  password: props.entry?.password || '',
  notes: props.entry?.notes || '',
  group: props.entry?.group || '默认分组',
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
  if (!form.group) form.group = '默认分组'
  if (!form.category) form.category = '其他'
  emit('save', { ...form })
}
</script>

<style scoped>
.form-wrap {
  padding: 0;
}

/* 顶栏 */
.form-head {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 700;
  color: var(--accent-red);
  padding: 0 0 16px;
  border-bottom: 1px solid var(--border);
  margin-bottom: 16px;
}
.head-icon { font-size: 20px; }

/* 表单主体 */
.form-body {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.f-row {
  display: flex;
  gap: 12px;
}
.f-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.flex-1 { flex: 1; }
.f-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  letter-spacing: 0.3px;
}

/* 密码字段的操作按钮 */
.pwd-act {
  cursor: pointer;
  font-size: 14px;
  padding: 2px 4px;
  border-radius: 4px;
  transition: all 0.15s;
  user-select: none;
}
.pwd-act:hover { background: var(--accent-red-glow); }
.pwd-act.gen {
  background: var(--accent-red-glow);
  border-radius: 4px;
  padding: 2px 6px;
  margin-left: 2px;
}
.pwd-act.gen:hover { background: var(--accent-red-glow-strong); }

/* 底部按钮 */
.form-foot {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border);
}
.btn-cancel {
  --n-border: 1px solid rgba(255,255,255,0.1) !important;
}
.btn-save {
  min-width: 140px;
  font-weight: 600;
}
</style>
