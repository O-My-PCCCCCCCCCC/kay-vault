# 🔑 凯伊密码管家 · Kay Vault

> **English ([README.md](README.md))** ┃ **中文**

> 一个运行在 U 盘上的桌面密码保险箱。主密码 + AES-256-GCM + Argon2id 加密保护，插上即用，拔走即消失。

![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-3.x-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-6.x-3178C6?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)

---

## 📑 目录

- [🎯 应用场景](#-应用场景)
- [🔐 加密思路](#-加密思路)
- [🏗️ 技术架构](#-技术架构)
- [🚀 快速开始](#-快速开始)

---

## 🎯 应用场景

### 解决的问题

你有多组密码、API 密钥需要管理，但：

- **不想用浏览器记密码** — 换台电脑就没了，而且浏览器本身安全性存疑
- **不想把密码放云上** — 担心泄漏、审查、服务商跑路
- **经常用公用电脑** — 网吧、公司电脑、学校机房，不敢存密码
- **需要一个便携方案** — 插上 U 盘就能用，拔掉什么都不留

### 我们的方案

把程序和加密数据放在 U 盘上。不依赖云服务，不依赖操作系统账户，所有加密在本地完成。

```
U 盘结构:
  ├── KayVault.exe ← 程序本体
  └── .key-vault/  ← 加密数据目录（U 盘隐藏目录）
       ├── vault.enc       ← 🔒 密码库
       ├── apikeys.enc     ← 🔒 API 密钥
       ├── master.verify   ← 🔑 主密码校验标签
       └── config.json     ← ⚙️ 设置
```

> 建议配合 BitLocker / VeraCrypt 全盘加密，双层保护。

### 核心功能

| 功能 | 说明 |
|------|------|
| **🔑 密码库** | AES-GCM 加密存储，支持分组/分类管理、搜索、复制 |
| **🔐 API 密钥管理器** | 安全存储 AI 服务商（OpenAI、Anthropic、GitHub、Azure、DeepSeek 等）的密钥 |
| **🎲 SHA-PIN 密码生成器** | 基于 SHA-256 双向链的确定性密码生成算法，离线可用，不存储密码 |
| **💾 备份与还原** | 加密备份到可自定义路径，导入时校验源文件主密码 |
| **🔒 独立锁定** | 密码库和 API 密钥可分别锁定，互不干扰 |
| **⏱️ 自动锁定** | 1/5/15/30 分钟无操作自动锁定，回到锁屏页 |
| **🛡️ 防截屏** | 解锁后自动启用窗口防截屏保护，截图/录屏显示黑色 |
| **🪟 单实例运行** | 只能打开一个窗口，防止误操作重复启动 |

---

## 🔐 加密思路

### 整体架构

整个系统的安全根是**主密码**，一切保护都建立在这之上：

```
主密码 (如 "bbkb")
       │
       ▼
┌──────────────────────────────────────┐
│  Argon2id 密钥派生                    │
│  ┌─ 输入: 主密码 + 随机盐值(32 字节)  │
│  ├─ 参数: 内存 64MB, 迭代 3 次       │
│  ├─ 特点: 内存硬(memory-hard)        │
│  │        GPU/ASIC 无法加速破解       │
│  └─ 输出: 256-bit 派生密钥           │
└─────────────┬────────────────────────┘
              │
      ┌───────┴───────┐
      │               │
      ▼               ▼
  SHA-256(密钥)     AES-256-GCM
  (身份验证)        (数据加密)
      │               │
      ▼               ▼
 master.verify     vault.enc
                   apikeys.enc
```

**两层完全独立的保护：**

| 层 | 用途 | 存储位置 | 算法 |
|----|------|---------|------|
| **身份验证层** | 验证用户输入的密码是否正确 | `master.verify` | SHA-256(派生密钥) |
| **数据加密层** | 加密所有存储的密码和密钥 | `vault.enc` + `apikeys.enc` | AES-256-GCM |

验证层和加密层使用同一派生密钥，但前者不解密数据，后者不解密密码——各司其职。

### 算法选型理由

| 算法 | 用途 | 为什么选它 |
|------|------|-----------|
| **Argon2id** | 从主密码派生 256-bit 密钥 | 2015 年密码哈希竞赛冠军。**内存硬**：暴力破解需要大量内存，GPU/ASIC/FPGA 无法加速。Bitwarden、1Password、KeePass 均在使用。默认配置下每次派生约 300ms，每小时只能试约 12000 次，8 位复杂密码需要数十亿年 |
| **AES-256-GCM** | 加密存储数据 | **美国国家标准技术局(NIST)认证**，256位密钥提供 2^256 种组合。GCM 模式提供认证加密 (Authenticated Encryption)，同时保证**机密性**和**完整性**，任何篡改都会被检测到。银行、政府机密文件同一标准 |
| **SHA-256** | 密码校验标签 | 用于验证主密码，不解密数据。速度快、抗碰撞，作为身份验证摘要足够安全 |

### 文件格式详解

**vault.enc / apikeys.enc：**

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
├─────────────────────────────────────────────────────────────────┤
│                          Salt (32 字节)                          │
│  用于 Argon2id 密钥派生。随机生成，每个文件不同。                  │
├─────────────────────────────────────────────────────────────────┤
│                         Nonce (12 字节)                          │
│  每次加密随机生成。AES-GCM 初始化向量，确保相同明文每次密文不同。  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                       Ciphertext (变长)                          │
│                      AES-256-GCM 加密后的密文                     │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                      Authentication Tag (16 字节)                │
│                      GMAC 认证标签，防篡改检测                     │
└─────────────────────────────────────────────────────────────────┘
```

**master.verify：**

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
├─────────────────────────────────────────────────────────────────┤
│                          Salt (32 字节)                          │
│  与 vault.enc 中的 salt 相同。用于 Argon2id 密钥派生。            │
├─────────────────────────────────────────────────────────────────┤
│                     SHA-256(密钥) (32 字节)                      │
│  派生密钥的 SHA-256 摘要。登录时比对，验证密码是否正确。           │
└─────────────────────────────────────────────────────────────────┘
```

> 注：`vault.enc` 和 `apikeys.enc` 使用**相同的派生密钥**（同一主密码 + 相同 salt）。所以修改主密码时两者都需要重新加密。

### 会话密钥机制

密码不在前端停留，这是整个设计中最重要的安全决策：

```
第一阶段 ── 登录 (只在解锁时执行一次)
═══════════════════════════════════════════
                    │ 用户输入 "bbkb"
                    ▼
┌─────────────────────────────────────────┐
│ Rust 后端 session_login(password)       │
│                                         │
│  1. 读 master.verify → 取出 salt + 摘要  │
│  2. Argon2id(password, salt) → 派生密钥  │
│  3. SHA-256(派生密钥) 比对 master.verify │
│  4. 一致 → 密码正确                      │
│  5. 派生密钥 → 存 HashMap               │
│     { session_id: "a1b2c3...", key }    │
│  6. 返回 session_id 给前端              │
└─────────────────────────────────────────┘
                    │ 前端保存 session_id
                    ▼

第二阶段 ── 操作 (每次数据读写)
═══════════════════════════════════════════
                    │ 前端 invoke('vault_load',
                    │       { sessionId: "a1b2c3..." })
                    ▼
┌─────────────────────────────────────────┐
│ Rust 后端 vault_load(session_id)        │
│                                         │
│  1. HashMap.get(session_id) → 得到密钥  │
│  2. 读 vault.enc → 跳过 salt(32B)      │
│  3. AES-256-GCM 解密                    │
│  4. 返回明文 JSON 给前端                │
│                                         │
│  注意: 不再跑 Argon2id，不再传密码       │
└─────────────────────────────────────────┘

第三阶段 ── 锁定
═══════════════════════════════════════════
                    │ 前端调用 session_lock
                    ▼
┌─────────────────────────────────────────┐
│ Rust 后端 session_lock(session_id)      │
│                                         │
│  HashMap.remove(session_id)             │
│  → 派生密钥从内存消失                   │
│  → 任何后续操作都返回                   │
│    "未登录或会话已过期"                  │
└─────────────────────────────────────────┘
```

**为什么这比直接传密码好？**

| 对比项 | 直接传密码 | 会话密钥 |
|--------|-----------|---------|
| 前端内存里有什么 | 密码明文 `"bbkb"` | 随机字符串 `"a1b2c3..."` |
| 泄漏后果 | 密码被窃，所有数据可解密 | 需要同时拿到 session_id + 调用后端，但无法反推密码 |
| 每次操作开销 | Argon2id (~300ms) | HashMap 查表 (~0.001ms) |
| 锁定 = ? | 删前端变量（假的） | 后端删密钥（真的） |

### 备份与导入的加密校验

备份文件不是明文——它和密码库用的是同一套加密。所以"捡到备份文件"不等于"能看密码"：

```
备份操作 (备份到指定目录):
  源文件: vault.enc (AES-256-GCM 加密)
  操作:  直接复制到备份目录/vault-20260701.enc
  安全性: 文件本身就是加密的，没有主密码谁也解不开

导入操作 (从备份文件恢复):
  用户选文件 → 输入该文件对应的主密码
  ┌────────────────────────────────────────┐
  │ Rust 后端 import_from_file(path, pwd) │
  │                                        │
  │  1. 读 .enc 文件 → 取出 salt + 密文    │
  │  2. Argon2id(pwd, salt) → 派生密钥    │
  │  3. AES-256-GCM 解密                   │
  │  4. 解密失败 → "密码错误" ❌            │
  │  5. 解密成功 → 验证是 JSON 格式        │
  │  6. 复制到 ~/.key-vault/vault.enc     │
  │  7. "导入成功" ✅                      │
  └────────────────────────────────────────┘
```

**关键保护点**：第 4 步——不知道密码的人连导入这关都过不了，根本到不了解密那一步。

### 前端安全措施

| 措施 | 实现方式 | 目标 |
|------|---------|------|
| 密码不存前端 | 登录后只保留 `session_id`，密码从 Rust 内存中丢弃 | 防止 XSS/内存 dump 窃取密码 |
| 防截屏 | `getCurrentWindow().setContentProtected(true)` | 阻止恶意程序截屏窃取显示中的密码 |
| 防中文输入 | CSS `ime-mode: disabled` + HTML `spellcheck="false"` | 防止输入法干扰导致密码错误 |
| 自动锁定 | `setTimeout` 监听 mousedown/keydown/wheel/touchstart | 用户离开后自动保护 |
| 活动追踪 | 重置计时器，永不停止追踪 | 精确计算无操作时间 |
| 独立锁定 | 密码库/API 密钥可分别 UI 锁定 | 分模块保护，互不影响 |

### 攻击面分析

| 攻击场景 | 能否得手 | 详细原因 |
|---------|---------|---------|
| 物理窃取 U 盘，读取 vault.enc | ❌ 不能 | AES-256-GCM 加密，无密钥无法解密。密钥来自主密码 + Argon2id，不可直接获取 |
| 暴力破解主密码 | ❌ 极难 | Argon2id 内存硬 (64MB)，单次约 300ms。8 位混合密码空间 ~6×10^15，需要数百万年 |
| 量子计算机攻击 AES-256 | ❌ 不能 | AES-256 抗量子攻击，Grover 算法也只能将安全级降为 128 位，依然安全 |
| DMA 攻击读取 Rust 堆内存 | ⚠️ 有条件 | 需要物理访问 + 特定硬件。Windows 10+ 有 Kernel DMA Protection，已缓解 |
| 截屏/录屏捕获密码数据 | ❌ 不能 | `setContentProtected(true)` 启用后所有屏幕捕获返回黑色 |
| 窃取 session_id | ⚠️ 风险低 | 仅凭 session_id 无法解密，需同时接触运行中的 Rust 进程。且 session 会过期 |
| 窃取备份文件 | ❌ 不能 | 导入需校验备份文件的原始主密码，解密失败则拒绝导入 |
| 读取前端 JS 变量 | ❌ 不能 | session_id 无法反推密码，过期即废弃 |
| 物理接触已解锁程序 | ⚠️ 自动锁定保护 | 离开无操作达到设定时间后自动锁定 |

---

## 🏗️ 技术架构

### 系统架构

```
┌────────────────────────────────────────────────────────────┐
│                Tauri v2 桌面应用                            │
│                                                            │
│  ┌─────────────────────┐      ┌─────────────────────────┐  │
│  │   Vue.js 3 前端      │      │     Rust 后端            │  │
│  │                      │      │                         │  │
│  │   Pinia 状态管理     │      │  SessionManager         │  │
│  │   Vue Router         │◄─IPC─│  (密钥缓存)              │  │
│  │   Naive UI           │      │                         │  │
│  │   Vite (HMR)         │      │  crypto.rs              │  │
│  │                      │      │  vault.rs / api_keys.rs │  │
│  │   组件层              │      │  backup.rs / auth.rs    │  │
│  │   视图层              │      │  config.rs / sha_pin.rs │  │
│  └─────────────────────┘      └─────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

前后端严格分离：
- **前端**（Vue.js 3）— 界面渲染、用户交互、状态管理
- **后端**（Rust）— 所有加密操作、文件 I/O、会话管理
- **通信** — Tauri IPC（序列化为 JSON，类型安全）
- **核心决策**：加密密钥**永不**跨越 IPC 边界，只存在 Rust 进程堆中

### 核心 Rust 模块详解

| 模块 | 职责 | 关键公开函数 | 依赖 |
|------|------|-------------|------|
| **`session.rs`** | 用户会话管理，密钥生命周期 | `login()` `lock()` `get_key()` `change_password()` | `sha2`, `uuid` |
| **`crypto.rs`** | 所有加密原语 | `derive_key()` `encrypt()` `decrypt()` `generate_salt()` | `aes-gcm`, `argon2`, `rand` |
| **`vault.rs`** | 密码库对象持久化 | `load_vault(path, key)` `save_vault(path, vault, key)` | `serde_json` |
| **`api_keys.rs`** | API 密钥对象持久化 | `load_keys(path, key)` `save_keys(keys, path, key)` | `serde_json` |
| **`backup.rs`** | 备份文件管理 | `backup_vault()` `restore_vault()` `list_backups()` | `chrono` |
| **`auth.rs`** | USB 设备认证（备份授权） | `is_authorized()` `generate_key()` | `rand`, `hex` |
| **`config.rs`** | JSON 配置读写 | `load_config()` `save_config()` | `serde_json` |
| **`sha_pin.rs`** | 确定性密码生成算法 | `compute_with_len()` | `sha2` |

### 目录结构

```
kay-vault/
│
├── src/                          ← Vue.js 3 前端 (TypeScript)
│   ├── App.vue                   ← 根组件：锁屏/侧栏/统计/防截屏/自动锁定
│   ├── main.ts                   ← Vue 应用入口
│   ├── env.d.ts                  ← TypeScript 类型声明
│   │
│   ├── router/
│   │   └── index.ts              ← 路由配置（vault/api-keys/terminal/settings）
│   │
│   ├── stores/                   ← Pinia 状态管理
│   │   ├── app.ts                ← 应用级状态：sessionId、锁定开关、登录/登出
│   │   └── vault.ts              ← 密码库数据：条目列表、搜索过滤、CRUD 操作
│   │
│   ├── views/                    ← 页面视图（路由级别组件）
│   │   ├── VaultView.vue         ← 密码库主页：树状分组 + 卡片列表 + 搜索
│   │   ├── ApiKeysView.vue       ← API 密钥管理：按提供商分组 + 内联编辑
│   │   ├── TerminalView.vue      ← SHA-PIN 终端：标识输入 + 双向链输出
│   │   └── SettingsView.vue      ← 设置页：自动锁定/备份路径/改密码/设备认证
│   │
│   ├── components/               ← 可复用组件
│   │   ├── LockScreen.vue        ← 锁屏：密码输入 + 校验 + 警告提示
│   │   ├── PasswordForm.vue      ← 密码编辑弹窗：表单验证
│   │   ├── PasswordCard.vue      ← 密码卡片：展示/复制/显示隐藏
│   │   ├── PasswordGenerator.vue ← 随机密码生成器：长度/字符集配置
│   │   ├── BackupPanel.vue       ← 备份操作面板：备份/还原/导入
│   │   └── DeviceAuthList.vue    ← USB 设备认证：添加/移除
│   │
│   └── styles/
│       └── theme.css             ← 全局主题：CSS 变量（颜色/字体/圆角）
│
├── src-tauri/                    ← Rust 后端
│   ├── src/
│   │   ├── main.rs               ← Tauri 入口点
│   │   ├── lib.rs                ← 命令注册、路径管理、Tauri Builder 配置
│   │   ├── session.rs            ← 会话管理器
│   │   ├── crypto.rs             ← 加密原语
│   │   ├── vault.rs              ← 密码库持久化
│   │   ├── api_keys.rs           ← API 密钥持久化
│   │   ├── auth.rs               ← 设备认证
│   │   ├── backup.rs             ← 备份还原
│   │   ├── config.rs             ← 配置读写
│   │   └── sha_pin.rs            ← SHA-PIN 算法
│   │
│   ├── Cargo.toml                ← Rust 依赖（tauri, aes-gcm, argon2, sha2 等）
│   ├── tauri.conf.json           ← Tauri 应用配置（窗口/签名/安全策略）
│   ├── build.rs                  ← 构建脚本
│   └── capabilities/default.json ← Tauri 权限配置
│
├── docs/
│   └── encryption-design.md      ← 加密架构详细文档
│
├── 启动-调试程序.bat              ← Windows 一键启动脚本
├── package.json                  ← Node.js 依赖与脚本
├── vite.config.ts                ← Vite 打包配置
├── tsconfig.json                 ← TypeScript 配置（根）
├── tsconfig.app.json             ← TypeScript 配置（应用）
├── tsconfig.node.json            ← TypeScript 配置（Node 环境）
└── .gitignore                    ← Git 忽略规则
```

### 技术栈

| 层次 | 技术 | 版本 | 用途 |
|------|------|------|------|
| 前端框架 | Vue.js 3 | ^3.5 | 响应式 UI |
| 构建工具 | Vite | ^8.1 | 开发服务器 + 打包 |
| 类型系统 | TypeScript | ~6.0 | 类型安全 |
| UI 组件库 | Naive UI | ^2.44 | 深色主题组件 |
| 状态管理 | Pinia | ^3.0 | 前端状态 |
| 路由 | Vue Router | ^4.6 | SPA 路由 |
| 桌面框架 | Tauri v2 | ^2.11 | 原生窗口 + IPC |
| 后端语言 | Rust | 2021 | 加密 + 文件 I/O |
| 加密算法 | AES-256-GCM | 0.10 | 数据加密 |
| 密钥派生 | Argon2 | 0.5 | 密码→密钥 |
| 哈希 | SHA-2 | 0.10 | 校验标签 |

---

## 🚀 快速开始

### 系统要求

| 依赖 | 最低版本 | 用途 |
|------|---------|------|
| Windows | 10+ | 当前目标平台（macOS/Linux 开发中） |
| Node.js | 18+ | 前端构建与开发服务器 |
| Rust | 1.77+ | 后端编译 |
| Cargo 系统依赖 | — | [Tauri v2 系统依赖](https://v2.tauri.app/start/prerequisites/) |

### 从源码构建

```bash
# 1. 克隆仓库
git clone https://github.com/O-My-PCCCCCCCCCC/kay-vault.git
cd kay-vault

# 2. 安装前端依赖
npm install

# 3. 开发模式（Vite + Tauri 同时启动，支持热重载）
npm run tauri dev
# 这将会：
#   - 启动 Vite 开发服务器 (http://localhost:5173)
#   - 编译 Rust 后端（首次约 2-3 分钟）
#   - 打开 Tauri 原生窗口

# 4. 生产构建
npm run tauri build
# 输出到 src-tauri/target/release/bundle/
```

### 快速启动（调试用）

Windows 下双击 **`启动-调试程序.bat`**：

```
============================================
  * 凯伊密码管家 - 开发调试模式
============================================

[1/3] 清理残留进程...       ← 自动关掉旧的 Vite + 窗口
[2/3] 检查依赖...           ← 检测 node_modules
[3/3] 启动 Tauri...

  前端端口: http://localhost:5173

→ 调试完毕按 Ctrl+C 停止
→ 按任意键关闭窗口
```

### 开发命令参考

```bash
npm run dev          # 仅启动前端（Vite 开发服务器，无 Tauri）
npm run build        # TypeScript 类型检查 + 前端生产构建
npm run preview      # 预览构建产物
npm run type-check   # TypeScript 类型检查（不构建）
npm run tauri dev    # Tauri 开发模式（前端 + 后端）
npm run tauri build  # Tauri 生产构建
```

### 分支说明

| 分支 | 用途 | 状态 |
|------|------|------|
| `main` | 稳定版本，生产构建 | ✅ 可用 |
| `feat/terminal-and-api` | 当前开发分支，SHA-PIN 终端 + API 密钥管理 | 🚧 活跃开发 |
| `master` | 原默认分支（旧仓库遗留），内容同 `main` | 📦 存档 |

---

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

---

## 💬 维护说明

> **写给每一位使用者**

首先，感谢你关注并使用 Kay Vault 🎉

作者目前是一名 **准初三学生**，时间、精力和金钱都非常有限。同时，本项目是在 **AI 辅助编程** 的帮助下完成的——作者本人并不会编程，所有代码逻辑、安全设计和问题排查都离不开 AI 的协助。

这意味着：

- 🐢 **更新和维护速度可能会比较慢**——请多一些耐心和理解
- 🧠 **代码质量可能不如专业开发者**——如果你发现问题，欢迎提 Issue 或 PR
- 💰 **作者资金紧张**——服务器、域名、开发环境维护都需要成本

**如果你觉得这个半成品能帮到你，方便的话，请作者喝一瓶魔爪吧 🙏**（赞助链接见下方）

> 项目虽小，但作者会尽最大努力让它活下去。谢谢你的支持 ❤️

---

### ☕ 赞助作者

*赞助链接待补充——支付通道明天打通后就会放上来。*

*创建者 [追寻光的影](https://github.com/O-My-PCCCCCCCCCC) · 2026*
