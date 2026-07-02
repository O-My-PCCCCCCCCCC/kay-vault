<template>
  <div class="tpv">
    <h2 class="page-title">🔢 SHA-PIN 生成器</h2>

    <div class="tp-card">
      <div class="tp-row"><span class="tp-label">输入A</span><n-input v-model:value="i1" size="large" @keyup.enter="f2" /></div>
      <div class="tp-row"><span class="tp-label">输入B</span><n-input ref="i2ref" v-model:value="i2" size="large" @keyup.enter="run" /></div>
      <div class="tp-row">
        <span class="tp-label">位数</span>
        <n-radio-group v-model:value="len">
          <n-radio-button :value="4">4</n-radio-button>
          <n-radio-button :value="6">6</n-radio-button>
          <n-radio-button :value="8">8</n-radio-button>
        </n-radio-group>
      </div>
      <div class="tp-acts">
        <n-button type="primary" @click="run" :loading="busy" :disabled="!i1||!i2">生成</n-button>
        <n-button @click="clr">清除</n-button>
      </div>

      <n-divider v-if="r" style="margin:12px 0" />
      <div v-if="r" class="tp-result">
        <div class="tp-line">正向链: <span class="tp-hl">{{ r.forward_result }}</span></div>
        <div class="tp-line">反向链: <span class="tp-hl">{{ r.reverse_result }}</span></div>
        <div class="tp-out">
          <span class="tp-final">{{ r.final_password }}</span>
          <n-button size="small" @click="cpy">复制</n-button>
        </div>
      </div>
      <div v-if="err" class="tp-err">{{ err }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

const msg = useMessage()
const i1 = ref(''), i2 = ref(''), len = ref(6), busy = ref(false), err = ref('')
const r = ref<{ forward_result: string; reverse_result: string; final_password: string } | null>(null)
const i2ref = ref<HTMLInputElement | null>(null)
function f2() { i2ref.value?.focus?.() }
function clr() { i1.value=''; i2.value=''; r.value=null; err.value='' }
async function run() {
  if (!i1.value.trim() || !i2.value.trim()) return
  busy.value=true; err.value=''; r.value=null
  try { r.value = await invoke('sha_pin_run', { input1: i1.value, input2: i2.value, passwordLen: len.value }) }
  catch (e: any) { err.value = String(e) }
  finally { busy.value=false }
}
async function cpy() {
  if (!r.value) return
  try { await navigator.clipboard.writeText(r.value.final_password); msg.success('已复制') }
  catch { msg.error('复制失败') }
}
</script>

<style scoped>
.tpv { padding: 16px 20px; height: 100vh; overflow-y: auto; }
.page-title { font-size: 16px; font-weight: 700; color: var(--accent); margin-bottom: 16px; }
.tp-card { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius); padding: 16px; }
.tp-row { display: flex; align-items: center; gap: 12px; margin-bottom: 10px; }
.tp-label { color: var(--text-secondary); font-size: 13px; min-width: 56px; flex-shrink: 0; }
.tp-acts { display: flex; gap: 8px; margin-top: 4px; }
.tp-line { font-size: 13px; color: var(--text-secondary); margin-bottom: 6px; font-family: monospace; }
.tp-hl { color: var(--accent-red); font-weight: 600; }
.tp-out { margin-top: 10px; padding: 10px 14px; background: var(--bg-primary); border-radius: 6px; border-left: 3px solid var(--accent-red); display: flex; align-items: center; gap: 12px; }
.tp-final { flex: 1; font-family: monospace; font-size: 18px; font-weight: bold; color: #e5c07b; letter-spacing: 3px; }
.tp-err { color: var(--accent-red); font-size: 13px; margin-top: 8px; }
</style>
