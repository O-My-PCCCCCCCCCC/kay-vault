<template>
  <div class="genv">
    <div class="gen-box">
      <div class="gen-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">➜  ~/password-generator</span></div>
      <div class="gen-body">
        <div class="result-row">
          <input class="gen-out" :value="pwd" readonly />
          <button class="btn" @click="gen">🔄</button>
          <button class="btn ok" @click="cpy">📋 复制</button>
        </div>

        <div class="opt-row"><span class="ol">长度: {{ len }}</span><input type="range" min="4" max="128" v-model.number="len" class="slider" /></div>
        <div class="opt-row"><label class="oc"><input type="checkbox" v-model="up" /> 大写 (A-Z)</label><label class="oc"><input type="checkbox" v-model="low" /> 小写 (a-z)</label><label class="oc"><input type="checkbox" v-model="dig" /> 数字 (0-9)</label><label class="oc"><input type="checkbox" v-model="sym" /> 符号</label><label class="oc"><input type="checkbox" v-model="nosim" /> 排除相似</label></div>

        <div class="tip">💡 提示: 生成的密码只会存在你的剪贴板里，不会保存到服务器</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'

const msg = useMessage()
const pwd = ref('')
const len = ref(16)
const up = ref(true), low = ref(true), dig = ref(true), sym = ref(false), nosim = ref(false)
const U = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', L = 'abcdefghijklmnopqrstuvwxyz', D = '0123456789', S = '!@#$%^&*()_+-=[]{}|;:,.<>?', SIM = '0OIl1'

function generate() {
  let chars = ''
  if (up.value) chars += U
  if (low.value) chars += L
  if (dig.value) chars += D
  if (sym.value) chars += S
  if (nosim.value) chars = chars.split('').filter(c => !SIM.includes(c)).join('')
  if (!chars) { pwd.value = '请至少选择一种字符类型'; return }
  const arr = new Uint32Array(len.value)
  crypto.getRandomValues(arr)
  pwd.value = Array.from(arr).map(v => chars[v % chars.length]).join('')
}

function gen() { generate() }
async function cpy() {
  try { await navigator.clipboard.writeText(pwd.value); msg.success('已复制') }
  catch { msg.error('复制失败') }
}

onMounted(generate)
</script>

<style scoped>
.genv { height: 100vh; display: flex; align-items: center; justify-content: center; background: #0b0e14; font-family: 'Consolas','Courier New',monospace; }
.gen-box { width: 500px; border: 1px solid #2c313a; border-radius: 8px; overflow: hidden; }
.gen-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; }
.dot { width: 12px; height: 12px; border-radius: 50%; }
.red { background: #ff5f57; } .yellow { background: #ffbd2e; } .green { background: #28c840; }
.bar-text { margin-left: 8px; font-size: 12px; color: #5c6370; }
.gen-body { padding: 20px; display: flex; flex-direction: column; gap: 16px; }
.result-row { display: flex; gap: 8px; align-items: center; }
.gen-out { flex: 1; background: #1a1d27; border: 1px solid #2c313a; border-radius: 4px; padding: 10px 14px; font-family: inherit; font-size: 16px; color: #e5c07b; letter-spacing: 2px; text-align: center; }
.opt-row { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; }
.ol { color: #5c6370; font-size: 13px; min-width: 50px; }
.slider { flex: 1; accent-color: #61afef; height: 4px; max-width: 200px; }
.oc { display: flex; align-items: center; gap: 4px; color: #abb2bf; font-size: 12px; cursor: pointer; }
.oc input { accent-color: #98c379; }
.btn { background: #2c313a; color: #abb2bf; border: none; padding: 6px 14px; border-radius: 4px; font-family: inherit; font-size: 13px; cursor: pointer; white-space: nowrap; }
.btn:hover { background: #3b4048; color: #e5c07b; }
.btn.ok { background: #3b4048; color: #98c379; }
.btn.ok:hover { background: #4a5060; color: #e5c07b; }
.tip { font-size: 11px; color: #3b4048; border-top: 1px solid #1a1d27; padding-top: 12px; }
</style>
