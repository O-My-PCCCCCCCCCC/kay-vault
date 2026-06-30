<template>
  <div class="vault-view">
    <div class="top-bar">
      <n-input
        v-model:value="vault.searchQuery"
        placeholder="搜索密码..."
        clearable
        style="max-width: 360px"
      >
        <template #prefix><n-icon><SearchIcon /></n-icon></template>
      </n-input>
      <div class="top-actions">
        <n-select
          v-model:value="vault.selectedCategory"
          :options="categoryOptions"
          style="width: 140px"
          clearable
        />
        <n-button type="primary" @click="openCreate">
          <template #icon><n-icon><AddIcon /></n-icon></template>
          新增
        </n-button>
      </div>
    </div>

    <div v-if="vault.filteredEntries.length === 0 && !vault.loading" class="empty-state">
      <div class="empty-icon">🔑</div>
      <p>还没有密码呢…</p>
      <p class="empty-hint">需要凯伊的钥匙帮你保管吗？</p>
      <n-button type="primary" dashed @click="openCreate">创建第一个密码</n-button>
    </div>

    <n-spin v-else-if="vault.loading" class="loading-spin" />
    <div v-else class="card-grid">
      <PasswordCard
        v-for="entry in vault.filteredEntries"
        :key="entry.id"
        :entry="entry"
        @edit="openEdit"
        @delete="confirmDelete"
      />
    </div>

    <!-- 新建/编辑弹窗 -->
    <n-modal
      v-model:show="showForm"
      :title="editingEntry ? '编辑密码' : '新建密码'"
      preset="card"
      style="width: 520px"
    >
      <PasswordForm :entry="editingEntry" @save="onFormSave" @close="showForm = false" />
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useMessage, useDialog } from 'naive-ui'
import { Search20Filled as SearchIcon, Add20Filled as AddIcon } from '@vicons/fluent'
import { useVaultStore, type VaultEntry } from '../stores/vault'
import PasswordCard from '../components/PasswordCard.vue'
import PasswordForm from '../components/PasswordForm.vue'

const vault = useVaultStore()
const message = useMessage()
const dialog = useDialog()

const showForm = ref(false)
const editingEntry = ref<VaultEntry | null>(null)

const categoryOptions = computed(() =>
  vault.categories.map((c: string) => ({ label: c, value: c }))
)

function openCreate() {
  editingEntry.value = null
  showForm.value = true
}

function openEdit(entry: VaultEntry) {
  editingEntry.value = { ...entry }
  showForm.value = true
}

function onFormSave(entry: VaultEntry) {
  if (editingEntry.value) {
    vault.updateEntry(entry)
    message.success('已更新')
  } else {
    vault.addEntry(entry)
    message.success('已添加')
  }
  showForm.value = false
}

function confirmDelete(entry: VaultEntry) {
  dialog.warning({
    title: '确认删除',
    content: `确定要删除「${entry.name}」的密码吗？`,
    positiveText: '确认删除',
    negativeText: '取消',
    onPositiveClick: async () => {
      await vault.deleteEntry(entry.id)
      message.success('已删除')
    },
  })
}
</script>

<style scoped>
.vault-view {
  padding: 24px;
  height: 100vh;
  display: flex;
  flex-direction: column;
}
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}
.top-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
}
.empty-icon {
  font-size: 64px;
}
.empty-hint {
  color: var(--text-secondary);
  font-size: 14px;
}
.loading-spin {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}
.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 16px;
  overflow-y: auto;
  flex: 1;
  padding-bottom: 24px;
}
</style>
