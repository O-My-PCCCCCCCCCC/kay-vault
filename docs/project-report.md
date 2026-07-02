# 🔑 凯伊密码管家 · 项目报告

> 生成日期: 2026-07-02
> 当前版本: v0.1.0
> 分支: main
> 仓库: https://github.com/O-My-PCCCCCCCCCC/kay-vault

---

## 一、项目概述

一个运行在 U 盘上的桌面密码管理器。主密码 + AES-256-GCM + Argon2id 加密保护，插上即用，拔走即消失。

### 技术栈

| 层 | 技术 | 版本 |
|----|------|------|
| 桌面框架 | Tauri v2 | ^2.11 |
| 前端 | Vue.js 3 + TypeScript | ^3.5 |
| 构建 | Vite | ^8.1 |
| UI | Naive UI | ^2.44 |
| 状态管理 | Pinia | ^3.0 |
| 后端 | Rust 2021 | — |
| 加密 | AES-256-GCM + Argon2id | — |

---

## 二、当前功能清单

### ✅ 已实现

| 功能 | 状态 | 说明 |
|------|------|------|
| 密码库 CRUD | ✅ | 增删改查，分组/分类树状管理 |
| API 密钥管理 | ✅ | 15 个供应商预置，搜索筛选 |
| SHA-PIN 生成器 | ✅ | 确定性密码生成（输入A+B） |
| 随机密码生成器 | ✅ | 长度/字符集配置，历史记录 10 条 |
| 会话密钥机制 | ✅ | 密码不存前端，派生密钥缓存 Rust 内存 |
| 真锁定 | ✅ | 后端删除派生密钥，无法继续解密 |
| 自动锁定 | ✅ | 1/5/15/30 分钟无操作自动锁定 |
| 防截屏 | ✅ | `setContentProtected(true)` |
| 会话 TTL | ✅ | 5 分钟无操作自动过期 |
| 设备指纹 | ✅ | C 盘序列号绑定，换机器自动踢下线 |
| 心跳检测 | ✅ | 每 3 分钟确认 session 存活 |
| 备份还原 | ✅ | 可配置路径，导入需校验密码 |
| 独立锁定 | ✅ | 密码库/API 密钥可分别 UI 锁定 |
| 单实例运行 | ✅ | 只允许一个窗口 |
| 中英文双语 README | ✅ | 英文主文档 + 中文独立文件 |
| 加密架构文档 | ✅ | docs/encryption-design.md |

### 🚧 待完善

| 功能 | 状态 | 说明 |
|------|------|------|
| UI 界面 | 🚧 | 风格不统一，用户表示需要改进 |
| macOS/Linux | 📅 | 当前仅 Windows |
| 密码强度检测 | 📅 | 未实现 |
| 导出密码 | 📅 | 未实现 |
| 自动更新 | 📅 | 未实现 |

---

## 三、加密架构

```
主密码 ─→ Argon2id ─→ 256-bit 派生密钥
                          │
                    ┌─────┴─────┐
                    │           │
                SHA-256      AES-256-GCM
              (密码校验)     (数据加密)
                    │           │
                    ▼           ▼
              master.verify   vault.enc
                            apikeys.enc
```

详见 `docs/encryption-design.md`

---

## 四、安全措施

| 措施 | 实现 |
|------|------|
| 密码不存前端 | 登录后只保留 sessionId |
| 防暴力破解 | Argon2id 内存硬，单次 ~300ms |
| 防截屏 | `setContentProtected(true)` |
| 防中文输入 | `ime-mode: disabled` |
| 真锁定 | 后端删 HashMap 中的派生密钥 |
| 会话 TTL | 5 分钟无操作自动过期 |
| 设备指纹 | C 盘序列号比对 |
| 心跳 | 每 3 分钟校验 session 存活 |
| 备份导入校验 | 解密失败拒绝导入 |
| IPC 白名单 | capabilities 限制 main 窗口 |

---

## 五、项目结构

```
kay-vault/
├── src/                      # Vue.js 前端
│   ├── stores/               # Pinia 状态管理
│   ├── views/                # 页面（5个）
│   ├── components/           # 组件（6个）
│   └── styles/               # 主题
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── lib.rs            # 命令注册（21个命令）
│   │   ├── session.rs        # 会话管理
│   │   ├── crypto.rs         # 加密原语
│   │   ├── vault.rs          # 密码库持久化
│   │   ├── api_keys.rs       # API 密钥持久化
│   │   ├── backup.rs         # 备份还原
│   │   ├── auth.rs           # 设备认证
│   │   ├── config.rs         # 配置读写
│   │   └── sha_pin.rs        # SHA-PIN 算法
│   └── Cargo.toml            # 19 个依赖
├── docs/
│   ├── encryption-design.md  # 加密架构文档
│   └── project-report.md     # 本文件
├── README.md                 # 英文文档
└── README.zh.md              # 中文文档
```

---

## 六、提交历史（最近）

```
f254984 fix: 对话框插件 + 备份路径修复
3de1cdd fix: API 搜索增加备注字段
ccb885f feat: 设置页清理 + 生成记录持久化 + API备注 + PIN统一风格
7213e4f fix: 生成记录序号从 #1 开始
1ce43eb fix: 生成记录改为两列布局
9af8a22 chore: 历史标题改为「生成记录」
f57efd2 fix: 生成按钮改为带文字标签
1213148 Reapply "合并生成器"
0b79eff Revert "合并生成器"
84018d4 feat: 生成器合并 + 终端风格
1838e37 refactor: 合并生成器
0cf9f4b style: 密码生成器应用主题
```

共 **51 个提交**，从 6 月 30 日到 7 月 2 日。

---

## 七、启动方式

```bash
cd kay-vault
npm install
npm run tauri dev      # 开发模式
npm run tauri build    # 生产构建
```

Windows 下也可双击 `启动-调试程序.bat`。

---

*报告由 Claude 生成 · 2026-07-02*
