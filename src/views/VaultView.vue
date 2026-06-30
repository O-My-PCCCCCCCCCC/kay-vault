<template>
  <div class="vault-view">
    <!-- 密码库锁定遮罩 -->
    <div class="top-bar">
      <n-input v-model:value="vault.searchQuery" placeholder="🔍 搜索密码..." clearable size="large" style="width: 300px" />
      <div class="top-acts">
        <n-button type="primary" size="large" @click="openCreate">➕ 新增</n-button>
      </div>
    </div>

      <div v-if="vault.entries.length === 0 && !vault.loading" class="empty-state">
        <div class="empty-icon">🔑</div>
        <p>还没有密码呢…</p>
        <p class="empty-hint">需要凯伊的钥匙帮你保管吗？</p>
        <n-button type="primary" dashed @click="openCreate">创建第一个密码</n-button>
      </div>
      <n-spin v-else-if="vault.loading" class="loading-spin" />

      <div v-else class="main-panel">
        <div class="tree-panel">
          <div class="tree-header">分组</div>
          <div class="tree-item" :class="{ active: view === 'all' }" @click="view = 'all'">📦 全部 <span class="badge">{{ vault.entries.length }}</span></div>
          <div v-for="grp in vault.treeData" :key="grp.group">
            <div class="tree-item" :class="{ active: view === grp.group }" @click="selectGroup(grp.group)">📦 {{ grp.group }} <span class="badge">{{ itemCount(grp) }}</span></div>
            <template v-if="view === grp.group || infoOpen(grp.group)">
              <div v-for="cat in grp.categories" :key="cat.category" class="tree-item sub" :class="{ active: view === grp.group + '/' + cat.category }" @click="selectCat(grp.group, cat.category)">
                📁 {{ cat.category }} <span class="badge">{{ cat.items.length }}</span>
              </div>
            </template>
          </div>
        </div>
        <div class="list-panel">
          <div class="list-header">{{ listTitle }}</div>
          <div v-if="items.length === 0" class="list-empty">空</div>
          <div v-else class="list-body">
            <div v-for="entry in items" :key="entry.id" class="entry" @click="openEdit(entry)">
              <div class="e-icon">{{ entry.name.charAt(0).toUpperCase() }}</div>
              <div class="e-body">
                <div class="e-name">{{ entry.name }}</div>
                <div class="e-fields">
                  <span v-if="entry.username" class="ef" @click.stop="copy(entry.username, '账号已复制')">📧 {{ entry.username }}</span>
                  <span v-if="entry.url" class="ef link" @click.stop="openUrl(entry.url)">🔗 {{ entry.url }}</span>
                  <span class="ef">🔑 {{ showPwd === entry.id ? entry.password : '••••••••' }}<span class="ef-toggle" @click.stop="togglePwd(entry.id)">{{ showPwd === entry.id ? '🙈' : '👁️' }}</span></span>
                </div>
              </div>
              <div class="e-acts">
                <span class="ea" title="复制密码" @click.stop="copy(entry.password, '密码已复制')">📋</span>
                <span class="ea" title="编辑" @click.stop="openEdit(entry)">✏️</span>
                <span class="ea del" title="删除" @click.stop="confirmDelete(entry)">🗑️</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <n-modal v-model:show="showForm" :title="editingEntry ? '编辑密码' : '新建密码'" preset="card" style="width: 520px">
      <PasswordForm :entry="editingEntry" @save="onFormSave" @close="showForm = false" />
    </n-modal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'
import { useVaultStore, type VaultEntry } from '../stores/vault'
import PasswordForm from '../components/PasswordForm.vue'

const vault = useVaultStore()
const msg = useMessage()

const showForm = ref(false)
const editingEntry = ref<VaultEntry | null>(null)
const showPwd = ref<string | null>(null)
const view = ref('all')
const openGroups = ref(new Set<string>())

const listTitle = computed(() => view.value === 'all' ? '📦 全部密码' : '📂 ' + view.value.replace('/', ' › '))
const items = computed(() => {
  if (view.value === 'all') return vault.filteredEntries
  const parts = view.value.split('/')
  for (const g of vault.treeData) {
    if (parts.length === 2 && g.group === parts[0]) for (const c of g.categories) if (c.category === parts[1]) return c.items
    if (g.group === parts[0]) return g.categories.flatMap(c => c.items)
  }
  return []
})

function infoOpen(g: string) { return openGroups.value.has(g) || view.value.startsWith(g + '/') }
function selectGroup(g: string) { openGroups.value = new Set([g]); view.value = g }
function selectCat(g: string, c: string) { openGroups.value = new Set([g]); view.value = g + '/' + c }
function itemCount(grp: any) { return grp.categories.reduce((s: number, c: any) => s + c.items.length, 0) }
function togglePwd(id: string) { showPwd.value = showPwd.value === id ? null : id }

function openCreate() { editingEntry.value = null; showForm.value = true }
function openEdit(e: VaultEntry) { editingEntry.value = { ...e }; showForm.value = true }
function onFormSave(e: VaultEntry) {
  if (editingEntry.value) { vault.updateEntry(e); msg.success('已更新') }
  else { vault.addEntry(e); msg.success('已添加') }
  showForm.value = false
}

async function confirmDelete(e: VaultEntry) {
  if (window.confirm(`确定要永久删除「${e.name}」的所有密码数据吗？\n此操作不可撤销。`)) {
    await vault.deleteEntry(e.id); msg.success('已删除')
  }
}

async function copy(t: string, m: string) {
  try { await navigator.clipboard.writeText(t); msg.success(m) } catch { msg.error('复制失败') }
}
async function openUrl(url: string) {
  try { await invoke('open_url', { url }) } catch (e: any) { msg.error(String(e)) }
}
</script>

<style scoped>
.vault-view { padding: 14px 18px; height: 100vh; display: flex; flex-direction: column; }
.top-bar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; flex-shrink: 0; }
.top-acts { display: flex; align-items: center; gap: 8px; }
.empty-state { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; }
.empty-icon { font-size: 64px; }
.empty-hint { color: var(--text-secondary); font-size: 14px; }
.loading-spin { flex: 1; display: flex; align-items: center; justify-content: center; }

/* 主面板 */
.main-panel { flex: 1; display: flex; gap: 10px; overflow: hidden; }
.tree-panel { width: 200px; flex-shrink: 0; overflow-y: auto; padding: 4px; }
.tree-header { font-size: 10px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.5px; padding: 6px 8px 8px; border-bottom: 1px solid var(--border); margin-bottom: 4px; }
.tree-item { display: flex; align-items: center; gap: 5px; padding: 5px 8px; margin: 1px 0; border-radius: 5px; cursor: pointer; font-size: 13px; color: var(--text-secondary); transition: all 0.1s; border-left: 2px solid transparent; }
.tree-item:hover { background: var(--accent-red-glow); color: var(--text-primary); }
.tree-item.active { background: var(--accent-red-glow-strong); color: var(--accent-red); font-weight: 600; border-left-color: var(--accent-red); }
.tree-item.sub { padding-left: 28px; font-size: 12px; }
.badge { margin-left: auto; font-size: 10px; color: var(--text-muted); background: rgba(255,255,255,0.03); padding: 0 6px; border-radius: 6px; }
.list-panel { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
.list-header { font-size: 14px; font-weight: 600; color: var(--text-primary); padding: 2px 0 8px; flex-shrink: 0; }
.list-empty { color: var(--text-muted); padding: 20px; text-align: center; }
.list-body { flex: 1; overflow-y: auto; }

.entry { display: flex; align-items: flex-start; gap: 8px; padding: 7px 8px; cursor: pointer; border-bottom: 1px solid rgba(255,255,255,0.04); transition: background 0.1s; }
.entry:last-child { border-bottom: none; }
.entry:hover { background: rgba(230,57,70,0.03); }
.e-icon { width: 26px; height: 26px; border-radius: 6px; margin-top: 1px; flex-shrink: 0; background: linear-gradient(135deg, var(--accent-red), var(--accent-red-soft)); display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 11px; color: #fff; }
.e-body { flex: 1; min-width: 0; }
.e-name { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.e-fields { display: flex; flex-wrap: wrap; gap: 3px 12px; margin-top: 1px; }
.ef { font-size: 11px; color: var(--text-secondary); font-family: monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 180px; }
.ef.link { cursor: pointer; }
.ef.link:hover { color: var(--accent-blue); }
.ef-toggle { font-size: 10px; color: var(--text-muted); cursor: pointer; margin-left: 2px; font-family: sans-serif; }
.ef-toggle:hover { color: var(--text-primary); }

.e-acts { display: flex; gap: 2px; flex-shrink: 0; margin-top: 2px; }
.ea { font-size: 12px; cursor: pointer; padding: 2px 5px; border-radius: 3px; transition: all 0.12s; color: var(--text-muted); opacity: 0.35; }
.entry:hover .ea { opacity: 0.6; }
.ea:hover { opacity: 1 !important; background: var(--accent-red-glow); color: var(--text-primary); }
.ea.del:hover { background: rgba(230,57,70,0.12); color: var(--accent-red); }
</style>
