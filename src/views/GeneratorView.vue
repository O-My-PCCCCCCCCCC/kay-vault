<template>
  <div class="genv">
    <div class="gen-box">
      <div class="gen-bar"><span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span><span class="bar-text">➜  ~/password-generator</span></div>

      <!-- 控制区 -->
      <div class="gen-ctrl">
        <div class="out-row">
          <input class="gen-out" :value="pwd" readonly placeholder="点击生成" />
          <button class="btn" @click="gen" title="生成">🔄</button>
          <button class="btn ok" @click="cpy" :disabled="!pwd">📋 复制</button>
        </div>

        <div class="opt-row"><span class="ol">长度: {{ len }}</span><input type="range" min="4" max="128" v-model.number="len" class="slider" /></div>
        <div class="opt-row">
          <label class="oc"><input type="checkbox" v-model="up" /> 大写</label>
          <label class="oc"><input type="checkbox" v-model="low" /> 小写</label>
          <label class="oc"><input type="checkbox" v-model="dig" /> 数字</label>
          <label class="oc"><input type="checkbox" v-model="sym" /> 符号</label>
          <label class="oc"><input type="checkbox" v-model="nosim" /> 排除相似</label>
        </div>
      </div>

      <!-- 分割 -->
      <div class="gen-div">─── 在此页面生成的密码 ───</div>

      <!-- 历史记录 -->
      <div class="gen-hist">
        <div v-if="history.length === 0" class="hist-empty">还没有生成过密码</div>
        <div v-for="(item, i) in history" :key="i" class="hist-item" @click="cpFromHist(item)">
          <span class="hist-num">#{{ history.length - i }}</span>
          <span class="hist-pwd">{{ item }}</span>
          <span class="hist-cp">📋</span>
        </div>
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
  // 记入历史
  history.value.unshift(pwd.value)
  if (history.value.length > 15) history.value = history.value.slice(0, 15)
}

function gen() { generate() }
async function cpy() {
  if (!pwd.value) return
  try { await navigator.clipboard.writeText(pwd.value); msg.success('已复制') }
  catch { msg.error('复制失败') }
}
async function cpFromHist(t: string) {
  try { await navigator.clipboard.writeText(t); msg.success('已复制: ' + t.slice(0, 4) + '…') }
  catch { msg.error('复制失败') }
}

onMounted(generate)
</script>

<style scoped>
.genv { height: 100vh; display: flex; align-items: center; justify-content: center; background: #0b0e14; font-family: 'Consolas','Courier New',monospace; }
.gen-box { width: 520px; max-height: 90vh; display: flex; flex-direction: column; border: 1px solid #2c313a; border-radius: 8px; overflow: hidden; }
.gen-bar { display: flex; align-items: center; gap: 8px; padding: 9px 16px; background: #1a1d27; flex-shrink: 0; }
.dot { width: 12px; height: 12px; border-radius: 50%; }
.red { background: #ff5f57; } .yellow { background: #ffbd2e; } .green { background: #28c840; }
.bar-text { margin-left: 8px; font-size: 12px; color: #5c6370; }

/* 控制区 */
.gen-ctrl { padding: 16px 20px; display: flex; flex-direction: column; gap: 12px; flex-shrink: 0; background: #0b0e14; }
.out-row { display: flex; gap: 8px; align-items: center; }
.gen-out { flex: 1; background: #1a1d27; border: 1px solid #2c313a; border-radius: 4px; padding: 10px 14px; font-family: inherit; font-size: 16px; color: #e5c07b; letter-spacing: 2px; text-align: center; }
.gen-out::placeholder { color: #3b4048; letter-spacing: 0; }
.opt-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
.ol { color: #5c6370; font-size: 13px; min-width: 50px; }
.slider { flex: 1; accent-color: #61afef; height: 4px; max-width: 180px; }
.oc { display: flex; align-items: center; gap: 3px; color: #abb2bf; font-size: 12px; cursor: pointer; }
.oc input { accent-color: #98c379; }
.btn { background: #2c313a; color: #abb2bf; border: none; padding: 6px 14px; border-radius: 4px; font-family: inherit; font-size: 13px; cursor: pointer; white-space: nowrap; }
.btn:hover { background: #3b4048; color: #e5c07b; }
.btn:disabled { opacity: 0.4; cursor: default; }
.btn.ok { background: #3b4048; color: #98c379; }
.btn.ok:hover { background: #4a5060; color: #e5c07b; }

/* 分割 */
.gen-div { text-align: center; font-size: 11px; color: #2c313a; padding: 4px 0; flex-shrink: 0; }

/* 历史 */
.gen-hist { flex: 1; overflow-y: auto; padding: 4px 20px 16px; }
.hist-empty { text-align: center; color: #3b4048; padding: 20px; font-size: 13px; }
.hist-item { display: flex; align-items: center; gap: 8px; padding: 6px 8px; margin: 2px 0; border-radius: 4px; cursor: pointer; transition: background 0.1s; }
.hist-item:hover { background: #1a1d27; }
.hist-num { color: #5c6370; font-size: 10px; min-width: 24px; }
.hist-pwd { flex: 1; font-size: 13px; color: #abb2bf; letter-spacing: 1px; font-family: inherit; }
.hist-cp { font-size: 12px; opacity: 0; transition: opacity 0.15s; }
.hist-item:hover .hist-cp { opacity: 0.6; }
.hist-item:hover .hist-cp:hover { opacity: 1; }
</style>
