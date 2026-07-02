<template>
  <div class="genv">
    <h2 class="page-title">🎲 生成器</h2>

    <n-tabs v-model:value="tab" type="line" animated>
      <!-- ─── 随机密码 ─── -->
      <n-tab-pane name="random" tab="🎲 随机密码">
        <div class="gen-card">
          <div class="out-row">
            <n-input :value="pwd" readonly size="large" placeholder="点击生成" />
            <n-button @click="gen" circle><template #icon>🔄</template></n-button>
            <n-button type="primary" @click="cpy" :disabled="!pwd">复制</n-button>
          </div>
          <n-divider style="margin: 12px 0" />
          <div class="opt-row">
            <span class="opt-label">长度: {{ len }}</span>
            <n-slider v-model:value="len" :min="4" :max="128" style="width: 200px" />
          </div>
          <div class="opt-checks">
            <n-checkbox v-model:checked="up">大写 (A-Z)</n-checkbox>
            <n-checkbox v-model:checked="low">小写 (a-z)</n-checkbox>
            <n-checkbox v-model:checked="dig">数字 (0-9)</n-checkbox>
            <n-checkbox v-model:checked="sym">符号 (!@#$%...)</n-checkbox>
            <n-checkbox v-model:checked="nosim">排除相似 (0OIl1)</n-checkbox>
          </div>
        </div>

        <n-divider />

        <h3 style="margin: 0 0 8px; font-size: 14px; color: var(--text-secondary)">📋 此页面生成的密码</h3>
        <div v-if="history.length === 0" class="hist-empty">还没有生成过密码</div>
        <div v-else class="hist-list">
          <div v-for="(item, i) in history" :key="i" class="hist-item" @click="cpHist(item)">
            <span class="hist-idx">#{{ history.length - i }}</span>
            <span class="hist-text">{{ item }}</span>
            <span class="hist-btn">复制</span>
          </div>
        </div>
      </n-tab-pane>

      <!-- ─── SHA-PIN ─── -->
      <n-tab-pane name="sha" tab="🔢 SHA-PIN">
        <div class="gen-card">
          <div class="sp-row"><span class="sp-label">输入A</span><n-input v-model:value="spA" placeholder="github.com" size="large" @keyup.enter="focusB" /></div>
          <div class="sp-row"><span class="sp-label">输入B</span><n-input ref="spBRef" v-model:value="spB" placeholder="········" size="large" @keyup.enter="doSha" /></div>
          <div class="sp-row">
            <span class="sp-label">位数</span>
            <n-radio-group v-model:value="spLen">
              <n-radio-button :value="4">4</n-radio-button>
              <n-radio-button :value="6">6</n-radio-button>
              <n-radio-button :value="8">8</n-radio-button>
            </n-radio-group>
          </div>
          <div class="sp-acts">
            <n-button type="primary" @click="doSha" :loading="spBusy" :disabled="!spA||!spB">生成</n-button>
            <n-button @click="clearSha">清除</n-button>
          </div>

          <n-divider v-if="spResult" style="margin: 12px 0" />
          <div v-if="spResult" class="sp-result">
            <div class="sp-line">正向链: <span class="sp-hl">{{ spResult.forward_result }}</span></div>
            <div class="sp-line">反向链: <span class="sp-hl">{{ spResult.reverse_result }}</span></div>
            <div class="sp-out">
              <span>最终密码</span>
              <span class="sp-final">{{ spResult.final_password }}</span>
              <n-button size="small" @click="cpSha(spResult.final_password)">复制</n-button>
            </div>
          </div>
          <div v-if="spErr" class="sp-err">{{ spErr }}</div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import { invoke } from '@tauri-apps/api/core'

const msg = useMessage()
const tab = ref('random')

// 随机密码
const pwd = ref('')
const len = ref(16)
const up = ref(true), low = ref(true), dig = ref(true), sym = ref(false), nosim = ref(false)
const history = ref<string[]>([])
const U = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', L = 'abcdefghijklmnopqrstuvwxyz', D = '0123456789', S = '!@#$%^&*()_+-=[]{}|;:,.<>?', SIM = '0OIl1'

function generate() {
  let chars = ''
  if (up.value) chars += U
  if (low.value) chars += L
  if (dig.value) chars += D
  if (sym.value) chars += S
  if (nosim.value) chars = chars.split('').filter(c => !SIM.includes(c)).join('')
  if (!chars) { pwd.value = ''; return }
  const arr = new Uint32Array(len.value)
  crypto.getRandomValues(arr)
  pwd.value = Array.from(arr).map(v => chars[v % chars.length]).join('')
  history.value.unshift(pwd.value)
  if (history.value.length > 15) history.value = history.value.slice(0, 15)
}
function gen() { generate() }
async function cpy() {
  if (!pwd.value) return
  try { await navigator.clipboard.writeText(pwd.value); msg.success('已复制') } catch { msg.error('复制失败') }
}
async function cpHist(t: string) {
  try { await navigator.clipboard.writeText(t); msg.success('已复制') } catch { msg.error('复制失败') }
}

// SHA-PIN
const spA = ref(''), spB = ref(''), spLen = ref(6), spBusy = ref(false), spErr = ref('')
const spResult = ref<{ forward_result: string; reverse_result: string; final_password: string } | null>(null)
const spBRef = ref<HTMLInputElement | null>(null)
function focusB() { spBRef.value?.focus?.() }
function clearSha() { spA.value = ''; spB.value = ''; spResult.value = null; spErr.value = '' }
async function doSha() {
  if (!spA.value.trim() || !spB.value.trim()) return
  spBusy.value = true; spErr.value = ''; spResult.value = null
  try { spResult.value = await invoke('sha_pin_run', { input1: spA.value, input2: spB.value, passwordLen: spLen.value }) }
  catch (e: any) { spErr.value = String(e) }
  finally { spBusy.value = false }
}
async function cpSha(t: string) {
  try { await navigator.clipboard.writeText(t); msg.success('已复制') } catch { msg.error('复制失败') }
}

onMounted(() => { generate() })
</script>

<style scoped>
.genv { padding: 16px 20px; height: 100vh; overflow-y: auto; }
.page-title { font-size: 16px; font-weight: 700; color: var(--accent-red); margin-bottom: 16px; }
.gen-card { background: var(--bg-secondary); border: 1px solid var(--border); border-radius: var(--radius); padding: 16px; }
.out-row { display: flex; gap: 8px; align-items: center; }
.opt-row { display: flex; align-items: center; gap: 16px; }
.opt-label { color: var(--text-secondary); font-size: 13px; min-width: 60px; }
.opt-checks { display: flex; flex-wrap: wrap; gap: 12px; margin-top: 4px; }
.hist-empty { color: var(--text-muted); padding: 24px; text-align: center; }
.hist-list { display: flex; flex-direction: column; gap: 2px; }
.hist-item { display: flex; align-items: center; gap: 10px; padding: 6px 8px; border-radius: 4px; cursor: pointer; transition: background 0.12s; }
.hist-item:hover { background: var(--bg-secondary); }
.hist-idx { color: var(--text-muted); font-size: 11px; min-width: 28px; font-family: monospace; }
.hist-text { flex: 1; font-family: monospace; font-size: 13px; color: var(--text-primary); letter-spacing: 0.5px; }
.hist-btn { font-size: 11px; color: var(--accent-blue); opacity: 0; transition: opacity 0.15s; }
.hist-item:hover .hist-btn { opacity: 1; }
.hist-btn:hover { text-decoration: underline; }

/* SHA-PIN */
.sp-row { display: flex; align-items: center; gap: 12px; margin-bottom: 10px; }
.sp-label { color: var(--text-secondary); font-size: 13px; min-width: 56px; flex-shrink: 0; }
.sp-acts { display: flex; gap: 8px; margin-top: 4px; }
.sp-line { font-size: 13px; color: var(--text-secondary); margin-bottom: 6px; font-family: monospace; }
.sp-hl { color: var(--accent-red); font-weight: 600; }
.sp-out { margin-top: 10px; padding: 10px 14px; background: var(--bg-primary); border-radius: 6px; border-left: 3px solid var(--accent-red); display: flex; align-items: center; gap: 12px; }
.sp-final { flex: 1; font-family: monospace; font-size: 18px; font-weight: bold; color: #e5c07b; letter-spacing: 3px; }
.sp-err { color: var(--accent-red); font-size: 13px; margin-top: 8px; }
</style>
