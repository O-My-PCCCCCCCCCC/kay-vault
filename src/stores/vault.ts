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
  category: string
  created_at: string
  updated_at: string
}

export const useVaultStore = defineStore('vault', () => {
  const entries = ref<VaultEntry[]>([])
  const loading = ref(false)
  const searchQuery = ref('')
  const selectedCategory = ref('全部')
  const masterPassword = ref('')
  const error = ref('')

  const filteredEntries = computed(() => {
    let result = entries.value
    if (selectedCategory.value && selectedCategory.value !== '全部') {
      result = result.filter(e => e.category === selectedCategory.value)
    }
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

  const categories = computed(() => {
    const cats = new Set(entries.value.map(e => e.category))
    return ['全部', ...cats]
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
    entries, loading, searchQuery, selectedCategory, masterPassword, error,
    filteredEntries, categories,
    loadFromDisk, saveToDisk, addEntry, updateEntry, deleteEntry,
  }
})
