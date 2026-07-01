# 🔑 凯伊密码管家 · 加密架构文档

> 版本: v0.1.0 (开发中)
> 最后更新: 2026-07-01

---

## 项目简介

凯伊密码管家（Kay Vault）是一个运行在 U 盘上的桌面密码管理器。所有数据文件存储在 U 盘的隐藏目录中，随 U 盘携带，插到任何 Windows 电脑上即可使用。

### 部署位置

```
U 盘 (如 F:/)
├── KayVault/               ← 程序本体（可执行文件）
│   ├── key-vault.exe
│   ├── resources/
│   └── ...
└── .key-vault/             ← 加密数据目录（由程序自动创建）
    ├── vault.enc           ← 密码库（AES-256-GCM 加密）
    ├── apikeys.enc         ← API 密钥（AES-256-GCM 加密）
    ├── master.verify       ← 主密码校验标签
    └── config.json         ← 配置（明文）
```

> **建议**: U 盘启用 BitLocker / VeraCrypt 全盘加密，双层保护。

---

## 加密架构总览

```
用户输入主密码 "bbkb"
       │
       ▼
┌─────────────────────────────┐
│  Argon2id (抗暴力破解)       │
│  盐值 32B + 密码 → 256-bit  │
│  派生密钥                    │
└──────────┬──────────────────┘
           │
           ├──→ HMAC-SHA256(密钥) → master.verify
           │     (校验密码对不对，不解密数据)
           │
           ├──→ AES-256-GCM → vault.enc
           │     (密码库条目)
           │
           └──→ AES-256-GCM → apikeys.enc
                 (API 密钥)
```

---

## 详细组件

### 1. 密钥派生 — Argon2id

**文件**: `src-tauri/src/crypto.rs`

```rust
pub fn derive_key(password: &str, salt: &[u8]) -> Vec<u8> {
    // 使用 Argon2id 算法，从主密码 + 随机盐派生 256-bit 密钥
    // Argon2id 是 2015 年密码哈希竞赛冠军
    // - 抗 GPU/ASIC 暴力破解
    // - 抗时间侧信道攻击
    // - 内存硬 (memory-hard)，增加破解成本
}
```

- 每次登录跑一次，派生出的密钥缓存在 Rust 内存中（会话）
- 后续所有加密操作直接用这个密钥，不再重复跑 Argon2id

### 2. 数据加密 — AES-256-GCM

**文件**: `src-tauri/src/crypto.rs`

```rust
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    // AES-256-GCM (Galois/Counter Mode)
    // - 256 位密钥强度（NIST 标准）
    // - 自动生成随机 12 字节 nonce（每次加密都不同）
    // - 自带 16 字节认证标签，防篡改
    // 返回: [nonce(12B) | ciphertext | tag(16B)]
}
```

| 特性 | 值 |
|------|-----|
| 算法 | AES-256-GCM |
| 密钥长度 | 256 bit |
| Nonce | 12 字节随机（每次加密不同） |
| 认证标签 | 16 字节 GMAC |
| 标准 | NIST SP 800-38D |

### 3. 文件格式

**密码库文件** `vault.enc`:

```
┌─────────────────┬──────────────────┬──────────────────────┐
│  Salt (32B)     │  Nonce (12B)     │  Ciphertext + Tag    │
│ （兼容保留）    │ （每次解密跳过）  │  (AES-256-GCM)       │
└─────────────────┴──────────────────┴──────────────────────┘
```

**校验文件** `master.verify`:

```
┌─────────────────┬──────────────────┐
│  Salt (32B)     │  SHA-256(密钥)   │
│  用于 Argon2id  │  32 字节校验标签  │
└─────────────────┴──────────────────┘
```

> `vault.enc` 和 `apikeys.enc` 格式相同，使用同一派生密钥。

### 4. 会话管理

**文件**: `src-tauri/src/session.rs`

```
登录:
  用户输入密码
  → 从 master.verify 读取盐值
  → Argon2id 派生密钥
  → SHA-256(密钥) 比对 master.verify 中的标签
  → 一致 → 密码正确
  → 密钥存入 HashMap<session_id, 密钥>
  → 返回 session_id 给前端

后续操作:
  前端传 session_id
  → 查 HashMap 取密钥
  → 直接 AES 解密
  → 不再跑 Argon2id

锁定:
  前端调 session_lock
  → 从 HashMap 删除 session_id
  → 密钥消失，无法解密
```

### 5. 前端安全措施

| 措施 | 实现 | 文件 |
|------|------|------|
| 密码不存前端 | 登录后只保留 session_id | `src/stores/app.ts` |
| 防中文输入 | `ime-mode: disabled` | `LockScreen.vue` |
| 防截屏 | `setContentProtected(true)` | `App.vue` |
| 自动锁定 | 无操作 N 分钟 → logout | `App.vue` |

---

## 攻击面分析

| 攻击方式 | 防护 | 风险等级 |
|---------|------|---------|
| 偷走 vault.enc 暴力破解 | Argon2id 内存硬，每秒只能试几次 | 🟢 极低 |
| 偷走 vault.enc + apikeys.enc | AES-256-GCM，没密钥解不开 | 🟢 极低 |
| 物理接触运行中的程序 | 防截屏 + 自动锁定 | 🟡 中 |
| 读取前端 JS 内存 | 只有 sessionId，没有密码 | 🟢 低 |
| DMA 攻击读内存 | 密钥在 Rust 堆上，Windows 有 Kernel DMA Protection | 🟡 中 |
| 日志泄漏 | debug 模式可能记密码参数 | 🟡 开发期注意 |
| 备份文件被偷 | 导入时校验密码 | 🟢 低 |

---

## 代码结构

```
src-tauri/src/
├── lib.rs          # Tauri 入口 + 命令注册
├── main.rs         # 主函数
├── session.rs      # 会话管理器（密钥缓存 + 锁定）
├── crypto.rs       # Argon2id + AES-256-GCM
├── vault.rs        # 密码库加载/保存
├── api_keys.rs     # API 密钥加载/保存
├── auth.rs         # USB 设备认证（备份用）
├── backup.rs       # 备份与还原
├── config.rs       # 配置管理
└── sha_pin.rs      # SHA-PIN 密码生成算法（独立功能）

src/
├── stores/
│   ├── app.ts      # 应用状态（sessionId, 锁定, 自动锁定）
│   └── vault.ts    # 密码库状态（调用后端命令）
├── components/
│   ├── LockScreen.vue    # 锁屏（输入密码 → 登录）
│   └── BackupPanel.vue   # 备份面板
└── views/
    ├── VaultView.vue     # 密码库列表
    ├── ApiKeysView.vue   # API 密钥管理
    └── SettingsView.vue  # 设置（路径配置）
```

---

## 安全建议

1. **U 盘全盘加密**: BitLocker（Windows 专业版）或 VeraCrypt
2. **主密码**: 至少 8 位，包含大小写字母 + 数字 + 符号
3. **备份**: 定期备份 `~/.key-vault/` 目录
4. **丢失密码**: 无法找回，请妥善保管

---

*© 2026 追寻光的影. MIT License.*
