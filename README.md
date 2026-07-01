# 🔑 凯伊密码管家 · Kay Vault

> 一个运行在 U 盘上的桌面密码管理器，基于 AES-256-GCM + Argon2id 加密存储。

![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-3.x-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-6.x-3178C6?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)

---

## 🎯 应用场景

**随身携带的密码保险箱。** 整个程序运行在 U 盘上，插到任何 Windows 电脑即可使用，拔走即消失。

```
U 盘 /
├── KayVault/          ← 程序本体
└── .key-vault/        ← 加密数据（AES-256-GCM）
    ├── vault.enc      ← 密码库
    ├── apikeys.enc    ← API 密钥
    └── master.verify  ← 主密码校验
```

适合：
- **不信任的电脑** — 在公用电脑上安全使用密码
- **跨设备携带** — U 盘一插即用，不留痕迹
- **离线安全** — 所有加密在本地完成，不依赖网络

> 建议配合 U 盘全盘加密（BitLocker / VeraCrypt）使用，双层保护。

### 功能清单

| 功能 | 说明 |
|------|------|
| **🔑 密码库** | AES-GCM 加密存储，支持分组/分类管理、搜索、复制 |
| **🔐 API 密钥管理器** | 安全存储 AI 服务商（OpenAI、Anthropic、GitHub 等）的 API 密钥 |
| **🎲 SHA-PIN 密码生成器** | 基于 SHA-256 双向链的确定性密码生成算法，离线可用 |
| **💾 备份与还原** | 加密备份到指定目录，导入时校验密码 |
| **🔒 独立锁定** | 密码库和 API 密钥可单独锁定，互不干扰 |
| **⏱️ 自动锁定** | 支持 1/5/15/30 分钟无操作自动锁定 |
| **🪟 单实例运行** | 只允许一个窗口，防止重复打开 |

---

## 🔐 加密思路

### 数据流

```
用户输入主密码 "bbkb"
       │
       ▼
┌────────────────────────┐
│  Argon2id 密钥派生      │  ← 抗暴力破解，一次跑几百毫秒
│  主密码 + 随机盐(32B)   │
│  → 256-bit 派生密钥     │
└────────┬───────────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
SHA-256     AES-256-GCM
(校验标签)   (加密数据)
    │         │
    ▼         ▼
master.verify  vault.enc + apikeys.enc
```

### 密钥管理

采用**会话密钥机制**，密码不留在前端：

```
登录时:
  "bbkb" → Argon2id → 派生密钥 → 存 Rust 内存 → 返回 session_id
                                                      │
之后操作:                                                │
  前端传 session_id ────────────────────────────────────┘
  → Rust 查内存拿密钥 → AES 解密
  → 不再跑 Argon2id，不再传密码明文
```

| 保护措施 | 实现 |
|---------|------|
| 密码不存前端 | 登录后只保留随机 session_id |
| 防截屏 | `setContentProtected(true)`，截图全黑 |
| 防中文输入 | 密码框禁用输入法 (`ime-mode: disabled`) |
| 真锁定 | 后端删除内存中的派生密钥，不解密 |
| 防暴力破解 | Argon2id 内存硬，每秒只能试几次 |
| 自动锁定 | 无操作 N 分钟 → 自动锁定 |

> 详细加密设计请参见 [docs/encryption-design.md](docs/encryption-design.md)。

---

## 🏗️ 技术架构

### 前后端分离

```
Vue.js 3 + TypeScript ─── Tauri IPC ─── Rust 后端
  前端界面                     调用        加密 + 文件 I/O
  Pinia 状态管理                          会话管理
  Naive UI 组件库                         文件读写
```

### 目录结构

```
kay-vault/
├── src/                      # Vue.js 前端
│   ├── App.vue               # 根组件（锁屏 + 布局 + 自动锁定）
│   ├── main.ts               # 入口
│   ├── router/               # 路由配置
│   ├── stores/               # Pinia 状态管理
│   │   ├── app.ts            # 应用状态（session_id, 锁定）
│   │   └── vault.ts          # 密码库状态
│   ├── views/                # 页面视图
│   │   ├── VaultView.vue     # 密码库
│   │   ├── ApiKeysView.vue   # API 密钥
│   │   ├── TerminalView.vue  # SHA-PIN 终端
│   │   └── SettingsView.vue  # 设置页（备份路径、密码修改）
│   ├── components/           # 组件
│   │   ├── LockScreen.vue    # 锁屏
│   │   ├── PasswordForm.vue  # 密码编辑表单
│   │   ├── PasswordCard.vue  # 密码卡片
│   │   ├── PasswordGenerator.vue  # 随机密码生成器
│   │   ├── BackupPanel.vue   # 备份面板
│   │   └── DeviceAuthList.vue # 设备认证列表
│   └── styles/               # 样式
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── lib.rs            # Tauri 入口 + 命令注册
│   │   ├── main.rs           # 主函数
│   │   ├── session.rs        # 会话管理器（密钥缓存 + 锁定）
│   │   ├── crypto.rs         # Argon2id + AES-256-GCM
│   │   ├── vault.rs          # 密码库加载/保存
│   │   ├── api_keys.rs       # API 密钥加载/保存
│   │   ├── auth.rs           # USB 设备认证（备份用）
│   │   ├── backup.rs         # 备份与还原
│   │   ├── config.rs         # 配置管理（路径、自动锁定）
│   │   └── sha_pin.rs        # SHA-PIN 算法
│   └── Cargo.toml
├── docs/
│   └── encryption-design.md  # 加密架构详细文档
└── package.json
```

---

## 🚀 快速开始

### 系统要求

- **Node.js** 18+
- **Rust** 1.77+
- [Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/O-My-PCCCCCCCCCC/kay-vault.git
cd kay-vault

# 安装前端依赖
npm install

# 开发模式（热重载）
npm run tauri dev

# 生产构建
npm run tauri build
```

### 快速启动（调试用）

项目根目录提供了 Windows 一键启动脚本：

**`启动-调试程序.bat`** — 双击即可启动 Tauri 开发模式
- 自动检测依赖，自动清理旧进程
- 中文提示，调试友好
- 前端端口: `http://localhost:5173`

---

## 📄 许可证

[MIT](LICENSE)

---

*由 [追寻光的影](https://github.com/O-My-PCCCCCCCCCC) 制作*
