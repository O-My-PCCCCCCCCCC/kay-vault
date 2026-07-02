<template>
  <div class="terminal">
    <!-- 标题栏 -->
    <div class="term-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">➜  ~/sha-pin</span></div>

    <!-- 输入区 -->
    <div class="term-input">
      <div class="line"><span class="p">┌──(<span class="pu">guest</span>@<span class="ph">pin</span>)<span class="pd">-</span>[<span class="pp">~</span>]</span></div>
      <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--输入A</span> <input v-model="i1" class="ti" placeholder="输入A" @keyup.enter="f2" :disabled="busy" /></div>
      <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--输入B</span> <input ref="p2" v-model="i2" :type="sp ? 'text' : 'password'" class="ti" placeholder="········" @keyup.enter="run" :disabled="busy" /> <span class="eye" @click="sp = !sp">{{ sp ? '🙈' : '👁' }}</span></div>
      <div class="line"><span class="p">└─<span class="ps">$</span></span> <span class="pc">sha-pin</span> <span class="pf">--位数</span> <span class="lo" :class="{on:len===4}" @click="len=4">4</span><span class="lo" :class="{on:len===6}" @click="len=6">6</span><span class="lo" :class="{on:len===8}" @click="len=8">8</span> <span class="ll">位</span></div>
      <div class="line"><span class="p">└─<span class="ps">$</span></span> <button class="btn" @click="run" :disabled="busy||!i1||!i2">{{ busy ? '⏳' : '▶' }} 生成</button> <button class="btn sec" @click="clr">✕ 清</button></div>
    </div>

    <!-- 输出 -->
    <div class="term-out" ref="out">
      <div class="ol og">$ sha-pin --交互模式</div>
      <div class="ol od">SHA-256 双向链 · 三重指纹 · 聚合加密 密码生成器</div>
      <div v-if="shown" class="or">
        <div class="ol os">──────────────────────</div>
        <div class="ol oi">📌 输入A: {{ i1 }}</div>
        <div class="ol oi">🔐 输入B: {{ '*'.repeat(i2.length) }}</div>
        <div class="ol os">── 正向链 ──</div>
        <div class="ol od">结果: <span class="hl">{{ r?.forward_result }}</span></div>
        <div class="ol os">── 反向链 ──</div>
        <div class="ol od">结果: <span class="hl">{{ r?.reverse_result }}</span></div>
        <div class="ol os">═══ 最终结果 ═══</div>
        <div class="of"><span class="fl">密码:</span><span class="fv">{{ r?.final_password }}</span><button class="cp" @click="cpy">复制</button></div>
        <div class="ol ow">⚠️ 请牢记输入值</div>
      </div>
      <div v-if="err" class="ol oe">{{ err }}</div>
      <div class="ol"><span class="cs">{{ busy ? '⏳' : '❯' }}</span><span class="cb">_</span></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'

const msg = useMessage()
const i1 = ref(''), i2 = ref(''), sp = ref(false), busy = ref(false)
const len = ref(6), shown = ref(false)
const r = ref<{ forward_result: string; reverse_result: string; final_password: string } | null>(null)
const err = ref('')
const out = ref<HTMLElement | null>(null)
const p2 = ref<HTMLInputElement | null>(null)
function f2() { p2.value?.focus() }
function sd() { nextTick(() => out.value?.scrollTo({ top: out.value.scrollHeight })) }
function clr() { i1.value = ''; i2.value = ''; shown.value = false; r.value = null; err.value = '' }
async function run() {
  if (!i1.value.trim() || !i2.value.trim()) return
  busy.value = true; err.value = ''; r.value = null; shown.value = true; sd()
  try { r.value = await invoke('sha_pin_run', { input1: i1.value, input2: i2.value, passwordLen: len.value }) }
  catch (e: any) { err.value = String(e) }
  finally { busy.value = false; sd() }
}
async function cpy() {
  if (!r.value) return
  try { await navigator.clipboard.writeText(r.value.final_password); msg.success('已复制') }
  catch { msg.error('复制失败') }
}
</script>

<style scoped>
.terminal { height: 100vh; display: flex; flex-direction: column; background: #0b0e14; font-family: 'Consolas','Courier New',monospace; font-size: 14px; color: #abb2bf; }
.term-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; flex-shrink: 0; }
.dot { width: 12px; height: 12px; border-radius: 50%; }
.red { background: #ff5f57; } .yellow { background: #ffbd2e; } .green { background: #28c840; }
.bar-text { margin-left: 8px; font-size: 12px; color: #5c6370; }
.term-input { padding: 10px 20px 6px; background: #0b0e14; border-bottom: 1px solid #1a1d27; flex-shrink: 0; }
.line { display: flex; align-items: center; gap: 6px; margin-bottom: 3px; flex-wrap: wrap; }
.p { color: #5c6370; font-size: 12px; white-space: nowrap; flex-shrink: 0; }
.pu { color: #98c379; } .ph { color: #c678dd; } .pd { color: #5c6370; } .pp { color: #e5c07b; }
.ps { color: #98c379; margin-right: 4px; } .pc { color: #61afef; font-size: 13px; } .pf { color: #d19a66; font-size: 12px; }
.ti { background: transparent; border: none; outline: none; color: #abb2bf; font-family: inherit; font-size: 14px; width: 220px; border-bottom: 1px solid #2c313a; padding: 1px 4px; }
.ti:focus { border-bottom-color: #61afef; }
.ti::placeholder { color: #3b4048; }
.eye { cursor: pointer; font-size: 13px; user-select: none; }
.lo { display: inline-block; padding: 1px 10px; margin: 0 2px; font-size: 13px; color: #5c6370; cursor: pointer; border: 1px solid #2c313a; border-radius: 4px; transition: all 0.12s; user-select: none; }
.lo:hover { border-color: #61afef; color: #abb2bf; }
.lo.on { border-color: #e5c07b; color: #e5c07b; background: rgba(229,192,123,0.08); }
.ll { color: #5c6370; font-size: 12px; }
.btn { background: #2c313a; color: #abb2bf; border: none; padding: 4px 14px; border-radius: 4px; font-family: inherit; font-size: 12px; cursor: pointer; }
.btn:hover { background: #3b4048; color: #e5c07b; }
.btn:disabled { opacity: 0.4; cursor: not-allowed; }
.btn.sec { background: transparent; border: 1px solid #2c313a; }
.btn.sec:hover { border-color: #5c6370; }
.term-out { flex: 1; overflow-y: auto; padding: 8px 20px 20px; background: #0b0e14; }
.ol { font-size: 13px; line-height: 1.7; }
.og { color: #98c379; } .od { color: #5c6370; } .oi { color: #abb2bf; }
.os { color: #3b4048; margin: 4px 0; }
.or { margin-top: 4px; }
.hl { color: #e5c07b; font-weight: 600; }
.of { margin: 8px 0; padding: 8px 14px; background: #1a1d27; border-radius: 6px; border-left: 3px solid #e5c07b; display: flex; align-items: center; gap: 12px; }
.fl { color: #e5c07b; font-size: 13px; }
.fv { color: #ffd700; font-size: 20px; font-weight: bold; letter-spacing: 3px; flex: 1; }
.cp { background: #2c313a; color: #abb2bf; border: none; padding: 3px 10px; border-radius: 4px; cursor: pointer; font-family: inherit; font-size: 12px; }
.cp:hover { background: #3b4048; color: #e5c07b; }
.oe { color: #e06c75; } .ow { color: #5c6370; font-size: 12px; }
.cs { color: #98c379; } .cb { animation: blink 1s step-end infinite; }
@keyframes blink { 50% { opacity: 0; } }
</style>
