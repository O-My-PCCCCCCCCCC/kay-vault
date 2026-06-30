import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  const unlocked = ref(false)
  return { unlocked }
})
