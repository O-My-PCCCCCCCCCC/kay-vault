const CLEAR_DELAY = 30_000 // 30 秒后自动清空
let clearTimer: ReturnType<typeof setTimeout> | null = null

/** 复制文本到剪贴板，并设置自动清空计时器 */
export async function copySecure(text: string): Promise<void> {
  // 清掉上一次的计时器
  if (clearTimer) {
    clearTimeout(clearTimer)
    clearTimer = null
  }

  await navigator.clipboard.writeText(text)

  // 30 秒后自动清空
  clearTimer = setTimeout(async () => {
    try {
      await navigator.clipboard.writeText('')
    } catch {
      // 忽略
    }
    clearTimer = null
  }, CLEAR_DELAY)
}

/** 立即清空剪贴板（锁定/登出时调用） */
export async function clearClipboard(): Promise<void> {
  if (clearTimer) {
    clearTimeout(clearTimer)
    clearTimer = null
  }
  try {
    await navigator.clipboard.writeText('')
  } catch {
    // 忽略
  }
}
