<template>
  <div class="gen-view">
    <PasswordGenerator :visible="true" @close="onClose" @select="onSelect" />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import PasswordGenerator from '../components/PasswordGenerator.vue'

const router = useRouter()

function onClose() { router.push('/') }
function onSelect(pwd: string) {
  navigator.clipboard.writeText(pwd)
  router.push('/')
}
</script>

<style scoped>
.gen-view { height: 100vh; display: flex; align-items: center; justify-content: center; background: var(--bg-primary); }
</style>
