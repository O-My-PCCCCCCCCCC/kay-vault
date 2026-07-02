<template>
  <div class="term">
    <div class="term-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">➜  ~/generator</span></div>

    <!-- 标签切换 -->
    <div class="tab-bar">
      <span class="tab" :class="{on:tab==='random'}" @click="tab='random'">🎲 随机密码</span>
      <span class="tab" :class="{on:tab==='sha'}" @click="tab='sha'">🔢 SHA-PIN</span>
    </div>

    <!-- ─── 随机密码 ─── -->
    <div v-if="tab==='random'" class="term-body">
      <div class="sec">
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">gen</span> <span class="pf">--生成</span> <button class="btn" @click="gen">▶ 生成</button> <span class="ll">{{ pwd || '点击生成' }}</span> <button class="btn sec" @click="cpy" :disabled="!pwd">📋 复制</button></div>
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">gen</span> <span class="pf">--长度</span> <span class="lo" :class="{on:len===8}" @click="len=8">8</span><span class="lo" :class="{on:len===16}" @click="len=16">16</span><span class="lo" :class="{on:len===24}" @click="len=24">24</span><span class="lo" :class="{on:len===32}" @click="len=32">32</span><span class="lo" :class="{on:len===64}" @click="len=64">64</span> <span class="ll">位 ({{ len }})</span></div>
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">gen</span> <span class="pf">--字符</span> <span class="lo" :class="{on:up}" @click="up=!up">大写</span><span class="lo" :class="{on:low}" @click="low=!low">小写</span><span class="lo" :class="{on:dig}" @click="dig=!dig">数字</span><span class="lo" :class="{on:sym}" @click="sym=!sym">符号</span><span class="lo" :class="{on:nosim}" @click="nosim=!nosim">排除相似</span></div>
      </div>

      <div class="sec-div">─── 历史 ───</div>

      <div class="sec hist">
        <div v-if="history.length===0" class="ol ow">还没有生成过密码</div>
        <div v-for="(item,i) in history" :key="i" class="hist-item" @click="cpHist(item)">
          <span class="hist-n">#{{ history.length-i }}</span>
          <span class="hist-p">{{ item }}</span>
          <span class="hist-c">📋</span>
        </div>
      </div>
    </div>

    <!-- ─── SHA-PIN ─── -->
    <div v-if="tab==='sha'" class="term-body">
      <div class="sec">
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--输入A</span> <input v-model="spA" class="ti" @keyup.enter="focusB" /></div>
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--输入B</span> <input ref="spBRef" v-model="spB" class="ti" @keyup.enter="doSha" /></div>
        <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--位数</span> <span class="lo" :class="{on:spLen===4}" @click="spLen=4">4</span><span class="lo" :class="{on:spLen===6}" @click="spLen=6">6</span><span class="lo" :class="{on:spLen===8}" @click="spLen=8">8</span> <span class="ll">位</span></div>
        <div class="line">
          <button class="btn" @click="doSha" :disabled="!spA||!spB||spBusy">{{ spBusy ? '⏳' : '▶' }} 生成</button>
          <button class="btn sec" @click="clearSha">✕ 清除</button>
        </div>
      </div>

      <div v-if="spResult" class="sec">
        <div class="sec-div">─── 正向链 ───</div>
        <div class="ol od">{{ spResult.forward_result }}</div>
        <div class="sec-div">─── 反向链 ───</div>
        <div class="ol od">{{ spResult.reverse_result }}</div>
        <div class="sec-div">═══ 最终结果 ═══</div>
        <div class="sp-out">
          <span class="sp-fv">{{ spResult.final_password }}</span>
          <button class="btn sec" @click="cpSha(spResult.final_password)">复制</button>
        </div>
      </div>
      <div v-if="spErr" class="ol oe">{{ spErr }}</div>
    </div>
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
function clearSha() { spA.value=''; spB.value=''; spResult.value=null; spErr.value='' }
async function doSha() {
  if (!spA.value.trim() || !spB.value.trim()) return
  spBusy.value=true; spErr.value=''; spResult.value=null
  try { spResult.value = await invoke('sha_pin_run', { input1: spA.value, input2: spB.value, passwordLen: spLen.value }) }
  catch (e: any) { spErr.value = String(e) }
  finally { spBusy.value=false }
}
async function cpSha(t: string) {
  try { await navigator.clipboard.writeText(t); msg.success('已复制') } catch { msg.error('复制失败') }
}

onMounted(() => generate())
</script>

<style scoped>
.term { height: 100vh; display: flex; flex-direction: column; background: #0b0e14; font-family: 'Consolas','Courier New',monospace; font-size: 14px; color: #abb2bf; }
.term-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; flex-shrink: 0; }
.dot { width: 12px; height: 12px; border-radius: 50%; }
.red { background: #ff5f57; } .yellow { background: #ffbd2e; } .green { background: #28c840; }
.bar-text { margin-left: 8px; font-size: 12px; color: #5c6370; }

/* 标签 */
.tab-bar { display: flex; gap: 0; border-bottom: 1px solid #1a1d27; flex-shrink: 0; padding: 0 16px; }
.tab { padding: 8px 16px; font-size: 13px; color: #5c6370; cursor: pointer; border-bottom: 2px solid transparent; transition: all 0.12s; }
.tab:hover { color: #abb2bf; }
.tab.on { color: #e5c07b; border-bottom-color: #e5c07b; }

.term-body { flex: 1; overflow-y: auto; padding: 10px 20px 20px; }
.sec { margin-bottom: 8px; }
.sec-div { color: #2c313a; font-size: 12px; margin: 6px 0; text-align: center; }
.line { display: flex; align-items: center; gap: 6px; margin-bottom: 4px; flex-wrap: wrap; }
.p { color: #5c6370; font-size: 12px; white-space: nowrap; flex-shrink: 0; }
.ps { color: #98c379; margin-right: 2px; }
.pc { color: #61afef; font-size: 13px; }
.pf { color: #d19a66; font-size: 12px; }
.ll { color: #abb2bf; font-size: 13px; }

/* 输入框 */
.ti { background: transparent; border: none; outline: none; color: #abb2bf; font-family: inherit; font-size: 14px; width: 260px; border-bottom: 1px solid #2c313a; padding: 1px 4px; }
.ti:focus { border-bottom-color: #61afef; }

/* 选项按钮 */
.lo { display: inline-block; padding: 2px 10px; margin: 0 2px; font-size: 12px; color: #5c6370; cursor: pointer; border: 1px solid #2c313a; border-radius: 4px; transition: all 0.12s; user-select: none; }
.lo:hover { border-color: #61afef; color: #abb2bf; }
.lo.on { border-color: #e5c07b; color: #e5c07b; background: rgba(229,192,123,0.08); }

/* 按钮 */
.btn { background: #2c313a; color: #abb2bf; border: none; padding: 4px 14px; border-radius: 4px; font-family: inherit; font-size: 12px; cursor: pointer; }
.btn:hover { background: #3b4048; color: #e5c07b; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }
.btn.sec { background: transparent; border: 1px solid #2c313a; }
.btn.sec:hover { border-color: #5c6370; }

/* 历史 */
.hist { padding: 2px 0; }
.ol { font-size: 13px; line-height: 1.7; }
.od { color: #5c6370; }
.oe { color: #e06c75; }
.ow { color: #3b4048; text-align: center; padding: 12px; }
.hist-item { display: flex; align-items: center; gap: 8px; padding: 3px 6px; margin: 1px 0; border-radius: 3px; cursor: pointer; }
.hist-item:hover { background: #1a1d27; }
.hist-n { color: #5c6370; font-size: 10px; min-width: 24px; }
.hist-p { flex: 1; font-size: 12px; color: #abb2bf; letter-spacing: 1px; }
.hist-c { font-size: 11px; opacity: 0; }
.hist-item:hover .hist-c { opacity: 0.5; }

/* SHA-PIN 结果 */
.sp-out { margin: 8px 0; padding: 8px 14px; background: #1a1d27; border-radius: 6px; border-left: 3px solid #e5c07b; display: flex; align-items: center; gap: 12px; }
.sp-fv { flex: 1; font-size: 18px; font-weight: bold; color: #ffd700; letter-spacing: 3px; font-family: inherit; }
</style>
