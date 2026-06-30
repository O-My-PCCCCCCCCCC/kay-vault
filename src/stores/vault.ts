import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface VaultEntry {
  id: string
  name: string
  url: string
  username: string
  password: string
  notes: string
  group: string
  category: string
  created_at: string
  updated_at: string
}

export const useVaultStore = defineStore('vault', () => {
  const entries = ref<VaultEntry[]>([])
  const loading = ref(false)
  const searchQuery = ref('')
  const masterPassword = ref('')
  const error = ref('')

  const filteredEntries = computed(() => {
    let result = entries.value
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase()
      result = result.filter(e =>
        e.name.toLowerCase().includes(q) ||
        e.username.toLowerCase().includes(q) ||
        e.url.toLowerCase().includes(q)
      )
    }
    return result
  })

  // 按分组 → 分类 分组的树结构
  const treeData = computed(() => {
    // 先按 group 分组
    const groupMap = new Map<string, Map<string, VaultEntry[]>>()
    for (const entry of filteredEntries.value) {
      const g = entry.group || '默认分组'
      const c = entry.category || '未分类'
      if (!groupMap.has(g)) groupMap.set(g, new Map())
      const catMap = groupMap.get(g)!
      if (!catMap.has(c)) catMap.set(c, [])
      catMap.get(c)!.push(entry)
    }

    // 转成树结构
    return Array.from(groupMap.entries()).map(([group, catMap]) => ({
      group,
      categories: Array.from(catMap.entries()).map(([category, items]) => ({
        category,
        items,
      })),
    }))
  })

  async function loadFromDisk(password: string) {
    loading.value = true
    error.value = ''
    try {
      masterPassword.value = password
      const data = await invoke<VaultEntry[]>('vault_load', { password })
      entries.value = data || []
    } catch (e: any) {
      error.value = String(e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function saveToDisk() {
    if (!masterPassword.value) return
    await invoke('vault_save', {
      entries: entries.value,
      password: masterPassword.value,
    })
  }

  async function addEntry(entry: VaultEntry) {
    entries.value.push(entry)
    await saveToDisk()
  }

  async function updateEntry(updated: VaultEntry) {
    const idx = entries.value.findIndex(e => e.id === updated.id)
    if (idx !== -1) {
      entries.value[idx] = updated
      await saveToDisk()
    }
  }

  async function deleteEntry(id: string) {
    entries.value = entries.value.filter(e => e.id !== id)
    await saveToDisk()
  }

  return {
    entries, loading, searchQuery, masterPassword, error,
    filteredEntries, treeData,
    loadFromDisk, saveToDisk, addEntry, updateEntry, deleteEntry,
  }
})
