# 🔑 凯伊密码管家 · Kay Vault

> 一个安全、简洁的桌面密码管理器，基于 SHA-PIN 算法生成确定性密码。

![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-3.x-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-6.x-3178C6?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)

---

## ✨ 功能

| 功能 | 说明 |
|------|------|
| **🔑 密码库** | AES-GCM 加密存储，支持分组/分类管理、搜索、复制 |
| **🔐 API 密钥管理器** | 安全存储和管理 AI 服务商（OpenAI、Anthropic、GitHub 等）的 API 密钥 |
| **🎲 SHA-PIN 密码生成器** | 基于 SHA-256 双向链的确定性密码生成算法，离线可用 |
| **💾 备份与还原** | 加密备份到本地目录，支持导入/导出 `.enc` 文件 |
| **🔒 独立锁定** | 密码库和 API 密钥可单独锁定，互不干扰 |
| **⏱️ 自动锁定** | 支持 1/5/15/30 分钟无操作自动锁定 |
| **🪟 单实例运行** | 只允许一个窗口，防止重复打开 |

## 🖼️ 截图

> *📸 截图待补充 — 欢迎贡献！*

## 🚀 快速开始

### 下载

从 [Releases](https://github.com/O-My-PCCCCCCCCCC/kay-vault/releases) 下载最新的安装包（.msi / .dmg）。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/O-My-PCCCCCCCCCC/kay-vault.git
cd kay-vault

# 安装前端依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

> **系统要求**: Node.js 18+、Rust 1.77+、以及 [Tauri 系统依赖](https://v2.tauri.app/start/prerequisites/)。

## 🏗️ 技术架构

```
kay-vault/
├── src/                      # Vue.js 前端
│   ├── App.vue               # 根组件（锁屏 + 主布局）
│   ├── main.ts               # 入口
│   ├── router/               # 路由配置
│   ├── stores/               # Pinia 状态管理
│   │   ├── app.ts            # 应用状态（解锁/锁定）
│   │   └── vault.ts          # 密码库状态
│   ├── views/                # 页面视图
│   │   ├── VaultView.vue     # 密码库
│   │   ├── ApiKeysView.vue   # API 密钥
│   │   ├── TerminalView.vue  # SHA-PIN 终端
│   │   └── SettingsView.vue  # 设置页
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
│   │   ├── vault.rs          # 密码库加解密
│   │   ├── auth.rs           # 设备认证
│   │   ├── backup.rs         # 备份还原
│   │   ├── config.rs         # 配置管理
│   │   ├── api_keys.rs       # API 密钥管理
│   │   ├── crypto.rs         # 加密工具
│   │   └── sha_pin.rs        # SHA-PIN 算法
│   └── Cargo.toml
└── package.json
```

### 安全设计

- **AES-256-GCM** 加密存储密码库
- **Argon2** 密钥派生，抗暴力破解
- **SHA-256 双向链** 确定性密码生成，不存储密码本身
- **主密码**：所有加密的根密钥，应用层不做缓存

## 🎲 SHA-PIN 算法

SHA-PIN 是一种**确定性密码生成算法**，核心思路：

1. 输入 **标识**（如 `github.com`）+ **主密码**
2. 通过 SHA-256 构建正向链和反向链
3. 双重指纹聚合后输出最终密码
4. 相同输入 → 相同输出，无需存储密码

支持 4/6/8 位长度，适合生成 PIN 码或强密码片段。

## 🔧 开发

```bash
# 安装依赖
npm install

# 开发模式（热重载）
npm run tauri dev

# 仅运行前端
npm run dev

# 生产构建
npm run tauri build

# 代码检查
npm run type-check
```

### 🪟 快速启动（调试用）

项目根目录提供了 Windows 一键启动脚本：

**`启动-调试程序.bat`** — 双击即可启动 Tauri 开发模式
- 自动检测 `node_modules`，缺失时自动 `npm install`
- 自动检测 Tauri CLI
- 中文提示，调试友好

## 🌿 开发分支

当前开发在 `feat/terminal-and-api` 分支：
- **TerminalView** — SHA-PIN 终端视图，交互式密码生成体验
- **ApiKeysView** — API 密钥管理面板

主分支为 `main`，稳定版从主分支构建。

## 📄 许可证

[MIT](LICENSE)

---

*由 [追寻光的影](https://github.com/O-My-PCCCCCCCCCC) 制作*
