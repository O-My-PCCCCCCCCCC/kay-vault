<template>
  <div class="akv">
    <div class="tb">
      <div class="search-wrap">
        <n-input v-model:value="q" placeholder="🔍 搜索..." clearable size="large" style="width:280px" @focus="sf=true" @blur="onBlur" />
        <div v-if="q && sf && sr.length>0" class="sp">
          <div v-for="k in sr" :key="k.idx" class="spi" @mousedown.prevent="jump(k.idx)">
            <div class="spn" v-html="hl(k.name)"></div>
            <div class="spm"><span class="spt">提供商</span> <span v-html="hl(k.provider)"></span></div>
          </div>
        </div>
        <div v-if="q && sf && sr.length===0" class="sp spe"><div class="spe-t">没有匹配的密钥</div></div>
      </div>
      <n-button type="primary" size="large" @click="add">➕ 新增</n-button>
    </div>

    <div v-if="keys.length === 0" class="empty"><div class="ei">🔐</div><p>还没有 API 密钥</p><n-button type="primary" dashed @click="add">添加第一个</n-button></div>

    <n-spin v-else-if="loading" class="spin" />

    <div v-else class="mp">
      <div class="tp">
        <div class="th">提供商</div>
        <div class="ti" :class="{on:sv==='all'}" @click="sv='all'">📦 全部 <span class="bd">{{ filtered.length }}</span></div>
        <div v-for="g in groups" :key="g.p" class="ti" :class="{on:sv===g.p}" @click="sv=g.p">📁 {{ g.p || '其他' }} <span class="bd">{{ g.items.length }}</span></div>
      </div>
      <div class="lp">
        <div class="lh">{{ sv === 'all' ? '📦 全部密钥' : '📁 '+sv }}</div>
        <div v-if="items.length===0" class="le">空</div>
        <div v-else class="lb" :class="{lock:app.apiLocked}">
          <div v-for="(k,i) in items" :key="i" class="er" :class="{on:hi===idx(k)}" @click="edit(i)">
            <div class="eico">{{ icon(k.provider) }}</div>
            <div class="eb">
              <div class="en" v-html="q ? hl(k.name) : k.name"></div>
              <div class="efs"><span class="ef" @click.stop="cpKey(k.key)">🔑 {{ sk === idx(k) ? k.key : mask(k.key) }} <span class="eft" @click.stop="ts(idx(k))">{{ sk === idx(k) ? '🙈' : '👁' }}</span></span><span v-if="k.base_url" class="ef link" @click.stop="cpKey(k.base_url)">🌐 {{ k.base_url }}</span></div>
            </div>
            <div class="ea"><span class="eab" @click.stop="cpKey(k.key)">📋</span><span class="eab del" @click.stop="del(idx(k))">🗑️</span></div>
          </div>
        </div>
      </div>
    </div>

    <n-modal v-model:show="fm" :title="ei>=0?'编辑':'新增'" preset="card" style="width:480px">
      <div class="fb">
        <div class="fr"><div class="fg f1"><label class="fl">📛 名称</label><n-input v-model:value="f.name" size="large" /></div><div class="fg f1"><label class="fl">🏢 提供商</label><n-auto-complete v-model:value="f.provider" :options="po" size="large" clearable /></div></div>
        <div class="fg"><label class="fl">🔑 密钥</label><n-input v-model:value="f.key" :type="fp?'text':'password'" size="large"><template #suffix><span class="fa" @click="fp=!fp">{{ fp?'🙈':'👁' }}</span></template></n-input></div>
        <div class="fg"><label class="fl">🌐 请求地址</label><n-input v-model:value="f.base_url" :placeholder="du" size="large" /></div>
        <div class="fg"><label class="fl">📝 备注</label><n-input v-model:value="f.notes" type="textarea" size="large" :autosize="{minRows:2,maxRows:4}" /></div>
      </div>
      <div class="fbtns"><n-button size="large" ghost @click="fm=false">取消</n-button><n-button size="large" type="primary" @click="save">保存</n-button></div>
    </n-modal>
  </div>

    <n-modal v-model:show="showDelAk" title="删除确认" preset="card" style="width: 340px">
      <p style="margin-bottom:12px;font-size:13px;color:var(--text-secondary)">删除需要验证主密码</p>
      <n-input v-model:value="delAkPwd" type="password" size="large" placeholder="输入主密码" @keyup.enter="doDelAk" />
      <div style="margin-top:12px;display:flex;justify-content:flex-end;gap:8px">
        <n-button size="small" @click="showDelAk = false">取消</n-button>
        <n-button size="small" type="primary" @click="doDelAk">确认删除</n-button>
      </div>
    </n-modal>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core"
import { copySecure } from "../utils/clipboard"
import { useMessage } from 'naive-ui'
import { useAppStore } from '../stores/app'

interface AK { name: string; key: string; provider: string; base_url: string; notes: string; created_at: string }
const app = useAppStore()
const msg = useMessage()

const keys = ref<AK[]>([]), q = ref(''), loading = ref(false), sk = ref<number|null>(null)
const sv = ref('all'), fm = ref(false), fp = ref(false), ei = ref(-1), sf = ref(false), hi = ref<number|null>(null)
const showDelAk = ref(false), delAkPwd = ref(''), delAkTarget = ref(-1)
const f = reactive({ name: '', key: '', provider: '', base_url: '', notes: '' })

const po = [
  {label:'🤖 OpenAI',value:'OpenAI'},
  {label:'🧠 Anthropic',value:'Anthropic'},
  {label:'🐙 GitHub',value:'GitHub'},
  {label:'🔮 Google AI',value:'Google AI'},
  {label:'🔵 Azure',value:'Azure'},
  {label:'☁️ Cloudflare',value:'Cloudflare'},
  {label:'🟣 Groq',value:'Groq'},
  {label:'🟢 DeepSeek',value:'DeepSeek'},
  {label:'🤗 HuggingFace',value:'HuggingFace'},
  {label:'⚫ Mistral AI',value:'Mistral AI'},
  {label:'🟠 Together AI',value:'Together AI'},
  {label:'🔴 Perplexity',value:'Perplexity'},
  {label:'🟡 ElevenLabs',value:'ElevenLabs'},
  {label:'🟤 Replicate',value:'Replicate'},
  {label:'⚪ 自定义',value:'自定义'},
]

const du = computed(() => {
  const u: Record<string,string> = {
    OpenAI:'https://api.openai.com/v1',
    Anthropic:'https://api.anthropic.com/v1',
    GitHub:'https://api.github.com',
    'Google AI':'https://generativelanguage.googleapis.com',
    Azure:'https://YOUR_RESOURCE.openai.azure.com',
    Cloudflare:'https://api.cloudflare.com/client/v4',
    Groq:'https://api.groq.com/openai/v1',
    DeepSeek:'https://api.deepseek.com/v1',
    HuggingFace:'https://api-inference.huggingface.co/v1',
    'Mistral AI':'https://api.mistral.ai/v1',
    'Together AI':'https://api.together.xyz/v1',
    Perplexity:'https://api.perplexity.ai',
    ElevenLabs:'https://api.elevenlabs.io/v1',
    Replicate:'https://api.replicate.com/v1',
  }
  return u[f.provider] || 'https://'
})

function icon(p: string) {
  const m: Record<string,string> = {
    OpenAI:'🤖',Anthropic:'🧠',GitHub:'🐙','Google AI':'🔮',
    Azure:'🔵',Cloudflare:'☁️',Groq:'🟣',DeepSeek:'🟢',
    HuggingFace:'🤗','Mistral AI':'⚫','Together AI':'🟠',
    Perplexity:'🔴',ElevenLabs:'🟡',Replicate:'🟤',自定义:'⚪'
  }
  return m[p]||'🔑'
}

// 搜索
const filtered = computed(() => {
  if (!q.value) return keys.value
  const l = q.value.toLowerCase()
  return keys.value.filter(k =>
    k.name.toLowerCase().includes(l) ||
    k.provider.toLowerCase().includes(l) ||
    k.key.toLowerCase().includes(l) ||
    k.base_url.toLowerCase().includes(l) ||
    (k.notes || '').toLowerCase().includes(l)
  )
})
const groups = computed(() => { const m = new Map<string,AK[]>(); for (const k of filtered.value) { const p = k.provider||'其他'; if (!m.has(p)) m.set(p,[]); m.get(p)!.push(k) }; return Array.from(m).map(([p,items])=>({p,items})) })
const items = computed(() => { if (sv.value==='all') return filtered.value; for (const g of groups.value) if (g.p === sv.value) return g.items; return [] })

// 搜索弹窗结果
const sr = computed(() => {
  if (!q.value) return []
  const l = q.value.toLowerCase()
  return keys.value.map((k,i) => [k,i] as [AK,number]).filter(([k]) =>
    k.name.toLowerCase().includes(l) ||
    k.provider.toLowerCase().includes(l) ||
    (k.notes || '').toLowerCase().includes(l)
  ).slice(0,20).map(([k,idx]) => ({name:k.name, provider:k.provider, idx}))
})

function hl(text: string): string {
  if (!q.value) return text
  const re = new RegExp(`(${q.value.replace(/[.*+?^${}()|[\]\\]/g,'\\$&')})`,'gi')
  return text.replace(re, '<mark class="hk">$1</mark>')
}
function onBlur() { setTimeout(() => { sf.value = false }, 200) }
function jump(idx: number) { hi.value = idx; sf.value = false }

function idx(k: AK) { return keys.value.indexOf(k) }
function mask(k: string) { if (!k||k.length<=8) return '••••••••'; return k.slice(0,4)+'••••'+k.slice(-4) }
function ts(i: number) { if (app.apiLocked) { msg.warning('API 密钥已锁定'); return }; sk.value = sk.value === i ? null : i }
async function load() { if (!app.sessionId) return; loading.value=true; try { keys.value = await invoke<AK[]>('api_keys_load',{sessionId:app.sessionId}) } catch{} finally{loading.value=false} }
async function saveAll() { if (!app.sessionId) return; await invoke('api_keys_save',{keys:keys.value,sessionId:app.sessionId}) }
function add() { if (app.apiLocked) { msg.warning("API 密钥已锁定"); return }; ei.value=-1; f.name=''; f.key=''; f.provider=''; f.base_url=''; f.notes=''; fm.value=true }
function edit(i: number) { if (app.apiLocked) { msg.warning("API 密钥已锁定"); return }; const k = items.value[i]; const idx = keys.value.indexOf(k); ei.value=idx; f.name=k.name; f.key=k.key; f.provider=k.provider; f.base_url=k.base_url; f.notes=k.notes||''; fm.value=true }
function save() { if (!f.name||!f.key) { msg.warning('请填写完整'); return }; const e: AK = { name:f.name, key:f.key, provider:f.provider||'自定义', base_url:f.base_url||du.value, notes:f.notes||'', created_at:new Date().toISOString() }; if (ei.value>=0) { keys.value[ei.value] = {...e, created_at: keys.value[ei.value].created_at} } else { keys.value.push(e) }; saveAll().then(()=>{msg.success('已保存');fm.value=false}) }
function del(i: number) { if (app.apiLocked) { msg.warning("API 密钥已锁定"); return }; delAkTarget.value = i; delAkPwd.value = ""; showDelAk.value = true }
async function doDelAk() { if (delAkTarget.value < 0 || !delAkPwd.value) return; try { await invoke('session_login', { password: delAkPwd.value }); keys.value.splice(delAkTarget.value, 1); await saveAll(); msg.success('已删除') } catch { msg.error('主密码错误，无法删除') } finally { showDelAk.value = false; delAkTarget.value = -1; delAkPwd.value = '' } }
async function cpKey(k: string) { if (app.apiLocked) { msg.warning('API 密钥已锁定'); return }; try { await copySecure(k); msg.success('已复制') } catch { msg.error('复制失败') } }
onMounted(load)
</script>

<style scoped>
.akv { padding: 14px 18px; height: 100vh; display: flex; flex-direction: column; }
.tb { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; flex-shrink: 0; }
.search-wrap { position: relative; }
.sp { position: absolute; top: 100%; left: 0; right: 0; z-index: 50; background: var(--bg-secondary); border: 1px solid var(--border); border-radius: 8px; margin-top: 4px; max-height: 280px; overflow-y: auto; box-shadow: 0 8px 32px rgba(0,0,0,0.3); }
.spe { padding: 20px; text-align: center; }
.spe-t { color: var(--text-muted); font-size: 13px; }
.spi { padding: 8px 12px; cursor: pointer; border-bottom: 1px solid var(--border); transition: background 0.08s; }
.spi:last-child { border-bottom: none; }
.spi:hover { background: var(--accent-red-glow); }
.spn { font-size: 13px; font-weight: 600; color: var(--text-primary); margin-bottom: 2px; }
.spm { font-size: 11px; color: var(--text-muted); }
.spt { display: inline-block; background: rgba(255,255,255,0.05); padding: 0 5px; border-radius: 3px; margin-right: 4px; }
:deep(.hk) { background: rgba(229,192,123,0.25); color: #e5c07b; border-radius: 2px; padding: 0 1px; font-style: normal; }
.er.on { background: var(--accent-red-glow); border-left: 2px solid var(--accent-red); }
.empty { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; }
.ei { font-size: 48px; }
.spin { flex: 1; display: flex; align-items: center; justify-content: center; }
.mp { flex: 1; display: flex; gap: 10px; overflow: hidden; }
.tp { width: 160px; flex-shrink: 0; overflow-y: auto; padding: 4px; }
.th { font-size: 10px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 1.5px; padding: 6px 8px 8px; border-bottom: 1px solid var(--border); margin-bottom: 4px; }
.ti { display: flex; align-items: center; gap: 5px; padding: 5px 8px; margin: 1px 0; border-radius: 5px; cursor: pointer; font-size: 13px; color: var(--text-secondary); transition: all 0.1s; border-left: 2px solid transparent; }
.ti:hover { background: var(--accent-red-glow); color: var(--text-primary); }
.ti.on { background: var(--accent-red-glow-strong); color: var(--accent-red); font-weight: 600; border-left-color: var(--accent-red); }
.bd { margin-left: auto; font-size: 10px; color: var(--text-muted); background: rgba(255,255,255,0.03); padding: 0 6px; border-radius: 6px; }
.lp { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
.lh { font-size: 14px; font-weight: 600; color: var(--text-primary); padding: 2px 0 8px; flex-shrink: 0; }
.le { color: var(--text-muted); padding: 20px; text-align: center; }
.lb { flex: 1; overflow-y: auto; }
.lb.lock { pointer-events: none; opacity: 0.5; }
.er { display: flex; align-items: flex-start; gap: 8px; padding: 7px 8px; cursor: pointer; border-bottom: 1px solid rgba(255,255,255,0.04); transition: background 0.1s; }
.er:hover { background: rgba(230,57,70,0.03); }
.eico { font-size: 18px; width: 28px; text-align: center; margin-top: 1px; flex-shrink: 0; }
.eb { flex: 1; min-width: 0; }
.en { font-size: 13px; font-weight: 600; color: var(--text-primary); }
.efs { display: flex; flex-wrap: wrap; gap: 2px 8px; margin-top: 1px; align-items: center; }
.ef { font-size: 11px; color: var(--text-secondary); font-family: monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 240px; }
.ef.link { cursor: pointer; }
.ef.link:hover { color: var(--accent-blue); }
.eft { cursor: pointer; font-family: sans-serif; padding: 0 2px; font-size: 11px; }
.ea { display: flex; gap: 2px; flex-shrink: 0; margin-top: 2px; }
.eab { font-size: 12px; cursor: pointer; padding: 2px 5px; border-radius: 3px; opacity: 0; transition: all 0.1s; color: var(--text-muted); }
.er:hover .eab { opacity: 0.5; }
.eab:hover { opacity: 1 !important; background: var(--accent-red-glow); color: var(--text-primary); }
.eab.del:hover { background: rgba(230,57,70,0.12); color: var(--accent-red); }
.fb { display: flex; flex-direction: column; gap: 12px; }
.fr { display: flex; gap: 12px; }
.fg { display: flex; flex-direction: column; gap: 4px; }
.f1 { flex: 1; }
.fl { font-size: 12px; font-weight: 500; color: var(--text-secondary); }
.fa { cursor: pointer; font-size: 13px; padding: 2px; border-radius: 3px; }
.fa:hover { background: var(--accent-red-glow); }
.fbtns { display: flex; justify-content: flex-end; gap: 10px; margin-top: 16px; padding-top: 14px; border-top: 1px solid var(--border); }
</style>
