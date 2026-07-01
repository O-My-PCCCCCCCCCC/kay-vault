# 🔑 凯伊密码管家 · Kay Vault

> 一个运行在 U 盘上的桌面密码保险箱。主密码 + AES-256-GCM + Argon2id 加密保护，插上即用，拔走即消失。

![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-3.x-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-6.x-3178C6?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)

---

## 🎯 应用场景

### 解决的问题

你有多组密码、API 密钥、PIN 码需要管理，但：
- 不想用浏览器记住密码（不安全、换电脑就没了）
- 不想把密码存在云上（怕泄漏、怕审查）
- 经常在公用电脑上登录（网吧、公司电脑、图书馆）

### 方案

把程序和加密数据都放在 U 盘上。插到 Windows 电脑就能用，拔掉 U 盘什么也不留。

```
U 盘结构:
  ├── KayVault.exe         ← 程序本体
  └── .key-vault/          ← 加密数据（U 盘隐藏目录）
       ├── vault.enc       ← 🔒 AES-256-GCM 加密的密码库
       ├── apikeys.enc     ← 🔒 同上加密的 API 密钥
       ├── master.verify   ← 🔑 主密码校验标签
       └── config.json     ← ⚙️ 设置（明文，不含密码）
```

### 核心功能

| 功能 | 说明 |
|------|------|
| **🔑 密码库** | AES-GCM 加密存储，分组/分类管理，搜索、复制、编辑 |
| **🔐 API 密钥管理器** | 管理 AI 服务商（OpenAI、Anthropic、GitHub、Azure 等）密钥 |
| **🎲 SHA-PIN 密码生成器** | 离线确定性密码生成，输入标识输出密码，不存密码本身 |
| **💾 备份与还原** | 加密备份到指定目录（可自定义路径），导入需校验主密码 |
| **🔒 独立锁定** | 密码库和 API 密钥可分别锁定，互不影响 |
| **⏱️ 自动锁定** | 无操作 1/5/15/30 分钟后自动锁回锁屏页 |
| **🛡️ 防截屏** | 启用后截图/录屏窗口区域显示为黑色 |
| **🪟 单实例运行** | 只允许一个窗口，防止重复打开 |

---

## 🔐 加密思路

### 总览

整个系统的安全根是**主密码**。所有数据都通过这一条链保护：

```
主密码 (如 "bbkb")
    │
    ▼
┌─────────────────────────────────┐
│ Argon2id 密钥派生                │
│  ├─ 输入: 主密码 + 随机盐(32B)   │
│  ├─ 算法: Argon2id (内存硬抗暴力) │
│  └─ 输出: 256-bit 派生密钥       │
└──────────┬──────────────────────┘
           │
     ┌─────┴─────┐
     │           │
     ▼           ▼
SHA-256       AES-256-GCM
(验证密码)    (加密数据)
     │           │
     ▼           ▼
master.verify  vault.enc     ← 密码库条目
               apikeys.enc   ← API 密钥
```

**两层保护：**
- **身份验证层** — `master.verify` 存 SHA-256(派生密钥)，用来验证密码对不对，不解密数据
- **数据加密层** — `vault.enc` 和 `apikeys.enc` 用 AES-256-GCM 加密，保证机密性和完整性

### 算法选型

| 算法 | 用途 | 为什么选它 |
|------|------|-----------|
| **Argon2id** | 从主密码派生 256-bit 密钥 | 2015 年密码哈希竞赛冠军。内存硬，GPU/ASIC 无法加速暴力破解。Bitwarden、1Password 都在用 |
| **AES-256-GCM** | 加密存储数据 | NIST 认证标准，256 位密钥强度。自带认证标签 (GMAC)，防篡改。银行级加密 |
| **SHA-256** | 生成密码校验标签 | 用于验证主密码是否正确，不解密数据，速度快 |

### 文件格式

**vault.enc / apikeys.enc：**
```
┌──────────┬──────────┬─────────────────────┐
│ Salt(32B)│Nonce(12B)│  Ciphertext + Tag   │
├──────────┴──────────┴─────────────────────┤
│ Salt 用于 Argon2id 派生密钥                │
│ Nonce 每次加密随机生成，确保同样数据每次不同 │
│ Ciphertext 是 AES-256-GCM 加密后的密文     │
│ Tag 是 16 字节认证标签，防篡改              │
└───────────────────────────────────────────┘
```

**master.verify：**
```
┌──────────┬──────────────────┐
│ Salt(32B)│  SHA-256 摘要(32B)│
├──────────┴──────────────────┤
│ 首次使用时创建，后续登录时比对 │
│ 摘要 = SHA-256(派生密钥)     │
└─────────────────────────────┘
```

### 会话密钥机制

密码不在前端停留，只在登录时用一次：

```
第一阶段 — 登录:
  用户输入 "bbkb"
  → Rust 后端收密码 → Argon2id 派生 256-bit 密钥
  → SHA-256 比对 master.verify → 一致则密码正确
  → 派生密钥存入 Rust 内存 HashMap
  → 返回随机 session_id ("a1b2c3d4...") 给前端
  → 前端只保存这个 session_id

第二阶段 — 操作:
  前端: invoke('vault_load', { sessionId: "a1b2c3d4..." })
  → 后端查 HashMap 拿派生密钥
  → 直接用 AES-256-GCM 解密
  → 不再跑 Argon2id（省时间），不再传密码（防泄漏）

第三阶段 — 锁定:
  前端调用 session_lock
  → 后端从 HashMap 删除该 session_id
  → 派生密钥从内存消失
  → 任何解密操作都报错 "未登录或会话已过期"
```

### 备份与导入的加密校验

备份文件和密码库文件是同一套加密，保证了"捡到备份文件也打不开"：

```
备份（export）:
  vault.enc ──直接复制──→ 备份目录/vault-20260701.enc
  ↑ 文件本身就是加密的，不需要额外加密

导入（import）:
  用户: 选择 .enc 文件 + 输入密码
  后端: 用密码尝试解密该文件
        ├── 解密成功 ✓ → 确认密码正确 → 复制到 vault.enc
        └── 解密失败 ✗ → "密码错误，拒绝导入"
```

### 前端安全措施

| 措施 | 实现方式 | 防止什么 |
|------|---------|---------|
| 密码不存前端 | session_id 替代 | 前端内存泄漏也拿不到密码 |
| 防截屏 | `setContentProtected(true)` | 恶意程序截图偷看密码 |
| 防中文输入 | `ime-mode: disabled` | 输入法干扰导致密码错误 |
| 自动锁定 | 无操作 N 分钟 → logout | 离开后别人继续使用 |
| 活动追踪 | 监听 mousedown/keydown/touchstart/wheel | 自动锁定计时器准确归零 |

### 攻击面分析

| 攻击场景 | 能否得手 | 原因 |
|---------|---------|------|
| 偷走 U 盘读 vault.enc | ❌ 不能 | AES-256-GCM 加密，没密钥解不开 |
| 暴力破解主密码 | ❌ 极难 | Argon2id 内存硬，每秒只能试几次 |
| 窃取内存中的密钥 | ⚠️ 有条件 | 密钥在 Rust 堆上，需本地提权 |
| 截屏偷看密码 | ❌ 不能 | 防截屏开启后截图全黑 |
| 导入备份文件破解 | ❌ 不能 | 导入需校验密码，错误拒接 |
| 读取前端 session_id | ⚠️ 风险低 | session_id 不能反推密码，无法解密 |
| 物理接触运行中的程序 | ⚠️ 自动锁定 | 无操作自动锁回锁屏页 |

---

## 🏗️ 技术架构

### 整体设计

```
┌──────────────────────────────────────────────────┐
│                  Tauri Desktop App               │
│  ┌─────────────────┐    ┌──────────────────────┐ │
│  │  Vue.js 3 前端   │    │     Rust 后端        │ │
│  │                  │    │                      │ │
│  │  Pinia 状态管理  │    │  SessionManager      │ │
│  │  Vue Router 路由 │    │  (密钥缓存 HashMap)  │ │
│  │  Naive UI 组件库 │    │                      │ │
│  │                  │    │  crypto.rs           │ │
│  │  LockScreen      │←─IPC──→  Argon2id + AES   │ │
│  │  VaultView      │    │                      │ │
│  │  ApiKeysView    │    │  vault.rs / backup   │ │
│  │  SettingsView   │    │  / auth / config     │ │
│  └─────────────────┘    └──────────────────────┘ │
└──────────────────────────────────────────────────┘
```

**前后端分离**：前端只负责界面渲染和用户交互，所有加密操作、文件读写都在 Rust 后端完成。前端通过 Tauri 的 `invoke` 调用后端命令。

**会话管理**：Rust 端持有一个 `SessionManager`，用 `Mutex<HashMap<String, Vec<u8>>>` 缓存登录后的派生密钥。前端只有 session_id，没有密码。

### 目录结构

```
kay-vault/
│
├── src/                         ← Vue.js 3 前端
│   ├── App.vue                  ← 根组件（锁屏/布局/自动锁定/防截屏）
│   ├── main.ts                  ← 应用入口
│   ├── router/index.ts          ← 路由配置
│   ├── stores/                  ← Pinia 状态管理
│   │   ├── app.ts               ← 全局状态（sessionId, 锁定开关）
│   │   └── vault.ts             ← 密码库数据（条目列表、搜索过滤）
│   ├── views/                   ← 页面视图
│   │   ├── VaultView.vue        ← 密码库主页（树状分组+卡片列表）
│   │   ├── ApiKeysView.vue      ← API 密钥管理（按提供商分组）
│   │   ├── TerminalView.vue     ← SHA-PIN 密码生成终端
│   │   └── SettingsView.vue     ← 设置页（自动锁定/备份路径/改密码）
│   ├── components/              ← 可复用组件
│   │   ├── LockScreen.vue       ← 锁屏（主密码输入+警告提示）
│   │   ├── PasswordForm.vue     ← 密码编辑表单弹窗
│   │   ├── PasswordCard.vue     ← 密码卡片展示
│   │   ├── PasswordGenerator.vue ← 随机密码生成器
│   │   ├── BackupPanel.vue      ← 备份还原操作面板
│   │   └── DeviceAuthList.vue   ← USB 设备认证列表
│   └── styles/theme.css         ← 全局主题样式变量
│
├── src-tauri/                   ← Rust 后端
│   ├── src/
│   │   ├── main.rs              ← 程序入口
│   │   ├── lib.rs               ← Tauri 命令注册 + 路径管理
│   │   ├── session.rs           ← 会话管理器（登录/锁定/换密码）
│   │   ├── crypto.rs            ← 加密原语（Argon2id + AES-256-GCM）
│   │   ├── vault.rs             ← 密码库读写（加密/解密/JSON 序列化）
│   │   ├── api_keys.rs          ← API 密钥读写（同上加密）
│   │   ├── auth.rs              ← USB 设备认证（备份授权）
│   │   ├── backup.rs            ← 备份还原（文件复制+目录管理）
│   │   ├── config.rs            ← 配置管理（JSON 读写）
│   │   └── sha_pin.rs           ← SHA-PIN 确定性密码生成算法
│   └── Cargo.toml               ← Rust 依赖清单
│
├── docs/
│   └── encryption-design.md     ← 加密架构详细文档
│
├── 启动-调试程序.bat              ← Windows 一键启动开发模式
├── package.json                 ← 前端依赖和脚本
├── vite.config.ts               ← Vite 构建配置
└── tsconfig*.json               ← TypeScript 配置
```

### Rust 后端核心模块说明

| 模块 | 职责 | 关键函数 |
|------|------|---------|
| `session.rs` | 会话管理 | `login()` / `lock()` / `get_key()` / `change_password()` |
| `crypto.rs` | 加解密 | `derive_key()` / `encrypt()` / `decrypt()` |
| `vault.rs` | 密码库持久化 | `load_vault()` / `save_vault()` |
| `api_keys.rs` | API 密钥持久化 | `load_keys()` / `save_keys()` |
| `backup.rs` | 备份还原 | `backup_vault()` / `restore_vault()` / `list_backups()` |

---

## 🚀 快速开始

### 系统要求

- **Windows 10+**（当前支持）、macOS、Linux（计划中）
- **Node.js** 18+（前端构建）
- **Rust** 1.77+（后端编译）
- [Tauri v2 系统依赖](https://v2.tauri.app/start/prerequisites/)

> 运行在 U 盘上不需要安装，直接拷贝可执行文件即可。

### 从源码构建

```bash
# 1. 克隆仓库
git clone https://github.com/O-My-PCCCCCCCCCC/kay-vault.git
cd kay-vault

# 2. 安装前端依赖
npm install

# 3. 开发模式（前端热重载 + Rust 实时编译）
npm run tauri dev

# 4. 生产构建（输出安装包到 src-tauri/target/release）
npm run tauri build
```

### 快速启动（调试用）

Windows 下双击 **`启动-调试程序.bat`**：
- 自动检测 `node_modules`，缺失则安装
- 自动清理旧进程（端口 5173 + 旧窗口）
- 启动 Tauri 开发模式
- 按 `Ctrl+C` 停止，窗口保持打开显示退出信息

### 开发命令

```bash
npm run dev          # 仅运行前端（Vite 开发服务器）
npm run build        # TypeScript 检查 + 前端构建
npm run preview      # 预览构建产物
npm run type-check   # TypeScript 类型检查
```

---

## 📄 许可证

[MIT](LICENSE)

---

*由 [追寻光的影](https://github.com/O-My-PCCCCCCCCCC) 制作*
