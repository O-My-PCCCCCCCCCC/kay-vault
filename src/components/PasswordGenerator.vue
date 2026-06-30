<template>
  <n-modal v-model:show="show" title="生成密码" preset="card" style="width: 420px">
    <div class="generator">
      <div class="result-row">
        <n-input v-model:value="generatedPassword" readonly size="large" />
        <n-button @click="regenerate" circle>
          <template #icon><n-icon><RefreshIcon /></n-icon></template>
        </n-button>
        <n-button type="primary" @click="confirm">使用</n-button>
      </div>

      <n-divider />

      <div class="options">
        <div class="option-row">
          <span class="option-label">长度: {{ length }}</span>
          <n-slider v-model:value="length" :min="4" :max="128" style="width: 200px" />
        </div>
        <n-checkbox v-model:checked="useUpper">大写字母 (A-Z)</n-checkbox>
        <n-checkbox v-model:checked="useLower">小写字母 (a-z)</n-checkbox>
        <n-checkbox v-model:checked="useDigits">数字 (0-9)</n-checkbox>
        <n-checkbox v-model:checked="useSymbols">符号 (!@#$%...)</n-checkbox>
        <n-checkbox v-model:checked="excludeSimilar">排除相似字符 (0OIl1)</n-checkbox>
      </div>
    </div>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ArrowSync20Filled as RefreshIcon } from '@vicons/fluent'

const props = defineProps<{ visible: boolean }>()
const emit = defineEmits<{ close: []; select: [password: string] }>()

const show = ref(false)
watch(() => props.visible, v => { if (v) { show.value = true; regenerate() } })
watch(show, v => { if (!v) emit('close') })

const generatedPassword = ref('')
const length = ref(16)
const useUpper = ref(true)
const useLower = ref(true)
const useDigits = ref(true)
const useSymbols = ref(false)
const excludeSimilar = ref(false)

const UPPER = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
const LOWER = 'abcdefghijklmnopqrstuvwxyz'
const DIGITS = '0123456789'
const SYMBOLS = '!@#$%^&*()_+-=[]{}|;:,.<>?'
const SIMILAR = '0OIl1'

function generatePassword(): string {
  let chars = ''
  if (useUpper.value) chars += UPPER
  if (useLower.value) chars += LOWER
  if (useDigits.value) chars += DIGITS
  if (useSymbols.value) chars += SYMBOLS

  if (excludeSimilar.value) {
    chars = chars.split('').filter(c => !SIMILAR.includes(c)).join('')
  }

  if (!chars) return ''

  const array = new Uint32Array(length.value)
  crypto.getRandomValues(array)
  let result = ''
  for (let i = 0; i < length.value; i++) {
    result += chars[array[i] % chars.length]
  }

  // 小彩蛋：极低概率生成凯伊彩蛋密码
  if (Math.random() < 0.001) {
    result = 'KeiIsBestGirl!!'
  }

  return result
}

function regenerate() {
  generatedPassword.value = generatePassword()
}

function confirm() {
  emit('select', generatedPassword.value)
  show.value = false
}
</script>

<style scoped>
.generator { display: flex; flex-direction: column; gap: 12px; }
.result-row { display: flex; gap: 8px; align-items: center; }
.options { display: flex; flex-direction: column; gap: 10px; }
.option-row { display: flex; align-items: center; gap: 16px; }
.option-label { color: var(--text-secondary); font-size: 13px; min-width: 60px; }
</style>
