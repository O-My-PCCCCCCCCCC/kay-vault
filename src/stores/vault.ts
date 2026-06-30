import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

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

  function addEntry(entry: VaultEntry) {
    entries.value.push(entry)
  }

  function updateEntry(updated: VaultEntry) {
    const idx = entries.value.findIndex(e => e.id === updated.id)
    if (idx !== -1) {
      entries.value[idx] = updated
    }
  }

  function deleteEntry(id: string) {
    entries.value = entries.value.filter(e => e.id !== id)
  }

  return {
    entries, loading, searchQuery, selectedCategory, masterPassword,
    filteredEntries, categories,
    addEntry, updateEntry, deleteEntry,
  }
})
