# 🔑 Kay Vault

> [中文](README.md) | **English**

> A portable desktop password manager that lives on a USB drive. Protected by AES-256-GCM + Argon2id. Plug in and use; unplug and vanish — no traces left on the host machine.

![Tauri](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri)
![Vue.js](https://img.shields.io/badge/Vue.js-3.x-4FC08D?logo=vue.js)
![TypeScript](https://img.shields.io/badge/TypeScript-6.x-3178C6?logo=typescript)
![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust)

---

## 📑 Table of Contents

- [Use Case](#-use-case)
- [Encryption Design](#-encryption-design)
- [Technical Architecture](#-technical-architecture)
- [Quick Start](#-quick-start)
- [Development](#-development)

---

## 🎯 Use Case

### The Problem

You have passwords, API keys, and PIN codes to manage, but:

- **Browser password managers are tied to a machine** — your credentials don't travel with you
- **Cloud-based managers require trust** — you must trust the provider's security, privacy policy, and business continuity
- **Shared/public computers are risky** — internet cafes, office workstations, school labs — you can't safely save credentials
- **You need true portability** — a solution that lives on a physical device you control, works on any Windows machine, and leaves no trace when removed

### The Solution

A self-contained desktop application that runs entirely from a USB drive. All data is encrypted locally on the drive, all cryptographic operations happen on-device, and nothing is sent to the cloud. When you unplug the drive, the application and its data are completely gone from the host machine.

```
USB Drive Layout:
  ├── KayVault.exe          ← Application binary
  └── .key-vault/            ← Encrypted data directory (hidden)
       ├── vault.enc        ← 🔒 Password vault (AES-256-GCM)
       ├── apikeys.enc      ← 🔒 API keys (AES-256-GCM)
       ├── master.verify    ← 🔑 Master password verification tag
       └── config.json      ← ⚙️ Settings (plaintext, no secrets)
```

> **Recommendation**: Combine with full-disk encryption (BitLocker or VeraCrypt) for defense in depth — the drive-level encryption protects at rest, while Kay Vault's encryption protects in use.

### Features

| Feature | Description |
|---------|-------------|
| **🔑 Password Vault** | AES-256-GCM encrypted storage with group/category management, search, copy, edit, and delete |
| **🔐 API Key Manager** | Securely store API keys for AI providers (OpenAI, Anthropic, GitHub, Azure, DeepSeek, Groq, Cloudflare, Google AI) |
| **🎲 SHA-PIN Generator** | Deterministic offline password generation using SHA-256 dual-chain algorithm. Same input → same output, no storage required |
| **💾 Backup & Restore** | Encrypted backup to a configurable directory. Import verifies the backup file's master password before allowing restoration |
| **🔒 Independent Locking** | Vault and API keys can be locked independently — one can stay unlocked while the other is secured |
| **⏱️ Auto-Lock** | Configurable inactivity timer (1/5/15/30 min). Automatically returns to the lock screen |
| **🛡️ Screenshot Protection** | Enables OS-level window content protection. Screenshots and screen recordings show a black window |
| **🪟 Single Instance** | Prevents multiple application windows from opening simultaneously |

---

## 🔐 Encryption Design

### High-Level Architecture

The entire security model is rooted in a single **master password**. All protection derives from it through a well-defined cryptographic chain:

```
Master Password (e.g. "bbkb")
       │
       ▼
┌──────────────────────────────────────────────┐
│  Argon2id Key Derivation                      │
│  ┌─ Inputs: Master password + random salt     │
│  ├─ Parameters: Memory 64MB, 3 iterations    │
│  ├─ Property: Memory-hard (ASIC/GPU/FPGA     │
│  │            resistant, cannot accelerate)  │
│  └─ Output: 256-bit Derived Key              │
└─────────────────────┬────────────────────────┘
                      │
              ┌───────┴───────┐
              │               │
              ▼               ▼
      SHA-256(derived key)  AES-256-GCM
      (Identity Layer)     (Data Layer)
              │               │
              ▼               ▼
        master.verify      vault.enc
                          apikeys.enc
```

**Two independent security layers:**

| Layer | Purpose | Storage | Algorithm |
|-------|---------|---------|-----------|
| **Identity Layer** | Verifies the user knows the correct master password without decrypting any data | `master.verify` | SHA-256(derived key) |
| **Data Layer** | Encrypts all stored password entries and API keys at rest | `vault.enc` + `apikeys.enc` | AES-256-GCM |

Both layers use the **same derived key**, but serve different purposes — verification does not touch encrypted data, and decryption does not verify the password. This separation ensures that a vulnerability in one layer does not compromise the other.

### Algorithm Selection Rationale

#### Argon2id — Key Derivation

**Purpose**: Transform a human-memorable master password into a cryptographically strong 256-bit key.

**Why Argon2id:**
- Winner of the **Password Hashing Competition (PHC)** in 2015
- **Memory-hard**: Requires a configurable amount of memory (default 64MB) to compute. This means parallel brute-force attempts require enormous amounts of RAM — a single GPU thread would need 64MB × thousands of threads = impractical amounts of memory bandwidth
- **GPU/ASIC/FPGA resistant**: Memory-hard functions cannot be accelerated by specialized hardware the way SHA-256 or bcrypt can
- **Industry standard**: Used by Bitwarden, 1Password, KeePass, and other leading password managers
- **Configurable cost**: Our default (64MB, 3 iterations) takes approximately 300ms per derivation on modern hardware, limiting brute-force to ~12,000 attempts per hour. An 8-character alphanumeric password (62^8 ≈ 2×10^14 combinations) would require billions of years to exhaust

**Security note**: The salt is 32 random bytes generated uniquely per file. Even if two users have the same master password, their derived keys will be completely different.

#### AES-256-GCM — Data Encryption

**Purpose**: Encrypt password entries and API keys at rest with authenticated encryption.

**Why AES-256-GCM:**
- **NIST standard** (FIPS PUB 197 / SP 800-38D)
- **256-bit key**: Provides 2^256 possible keys — far beyond any conceivable brute-force capability. Even quantum computers using Grover's algorithm would reduce this to 2^128, still secure
- **Authenticated Encryption (AEAD)**: GCM mode provides both **confidentiality** (nobody can read your data without the key) and **integrity** (nobody can tamper with your encrypted data without detection)
- **Random nonce**: A fresh 12-byte random nonce is generated for every encryption operation. This means:
  - Encrypting the same plaintext twice produces different ciphertext
  - Nonce reuse resistance (our random generation makes collision negligible at 2^96 space)
- **Built-in authentication tag**: The 16-byte GMAC tag detects any modification to the ciphertext. Decryption fails if the tag doesn't match

#### SHA-256 — Verification Tag

**Purpose**: Create a one-way fingerprint of the derived key for password verification.

**Why SHA-256:**
- **Collision-resistant**: No known practical collision attacks for SHA-256
- **Deterministic**: Same key always produces the same tag
- **Non-reversible**: The derived key cannot be recovered from the tag
- **Fast computation**: But this speed doesn't weaken security because:
  - The SHA-256 input is the **derived key** (already strengthened by Argon2id), not the raw password
  - An attacker would need to brute-force the Argon2id output (~2^256 space), which is computationally infeasible

### File Format Specification

#### vault.enc / apikeys.enc

Binary layout (big-endian):

```
Offset  Size  Field         Description
─────────────────────────────────────────────────────────────
 0       32    salt          Random bytes for Argon2id derivation.
                             Unique per file, generated on creation.

32       12    nonce         AES-GCM initialization vector.
                             Fresh random bytes per encryption.
                             Ensures semantic security (same plaintext
                             → different ciphertext).

44       N     ciphertext    AES-256-GCM encrypted payload.
                             Variable length. Contains JSON-serialized
                             VaultFile or ApiKey array.

44+N     16    tag           GMAC authentication tag.
                             Verifies ciphertext integrity.
                             Decryption fails if tag doesn't match.
```

**Total minimum size**: 44 bytes (empty vault with minimum padding) + 16 byte tag = 60 bytes. Real vaults are larger due to JSON content.

#### master.verify

```
Offset  Size  Field         Description
─────────────────────────────────────────────────────────────
 0       32    salt          Same salt as vault.enc.
                             Ensures the derived key is identical
                             for both verification and decryption.

32       32    tag           SHA-256(derived key).
                             Stored during first-time setup.
                             Compared on login to verify password.
```

### Session Key Mechanism

This is the most important security design decision in the application. The master password **never** resides in frontend memory after login. Here is the complete flow:

```
Phase 1 — Login (executed once, on unlock)
═══════════════════════════════════════════════════════════════

User Action:
  Enters master password in the LockScreen UI
       │  invoke('session_login', { password: "bbkb" })
       ▼
┌──────────────────────────────────────────────────────────────┐
│ Rust Backend — session::SessionManager::login()              │
│                                                               │
│  1. Read ~/.key-vault/master.verify                           │
│     → Extract salt (bytes 0-31) and stored_tag (bytes 32-63) │
│                                                               │
│  2. Key derivation:                                           │
│     derived_key = Argon2id(password, salt,                     │
│                          memory=64MB, iterations=3)           │
│                                                               │
│  3. Verification:                                             │
│     computed_tag = SHA-256(derived_key)                       │
│     if computed_tag ≠ stored_tag → return Err("密码错误")     │
│                                                               │
│  4. Session creation:                                         │
│     session_id = UUIDv4()  // e.g. "a1b2c3d4-e5f6-..."      │
│     sessions: HashMap::insert(session_id, SessionData {       │
│         key: derived_key,                                     │
│         created_at: Instant::now(),                           │
│     })                                                        │
│                                                               │
│  5. Return session_id to frontend                             │
│     // derived_key stays in Rust heap, never crosses IPC     │
└──────────────────────────────────────────────────────────────┘
       │  Frontend stores session_id in Pinia store
       │  Master password is immediately discarded from JS memory
       ▼

Phase 2 — Operations (every read/write)
═══════════════════════════════════════════════════════════════

User Action:
  Browses vault or views API keys
       │  invoke('vault_load', { sessionId: "a1b2c3d4-..." })
       ▼
┌──────────────────────────────────────────────────────────────┐
│ Rust Backend — vault_load(session_id)                        │
│                                                               │
│  1. Lookup:                                                   │
│     key = sessions.get(session_id)?.key                       │
│     if not found → return Err("未登录或会话已过期")           │
│                                                               │
│  2. Read vault.enc from disk                                  │
│     → Skip first 32 bytes (salt, retained for format compat) │
│                                                               │
│  3. Decrypt:                                                  │
│     plaintext = AES-256-GCM::decrypt(encrypted_data, key)     │
│                                                               │
│  4. Deserialize:                                              │
│     entries = serde_json::from_slice(plaintext)                │
│                                                               │
│  5. Return entries to frontend                                │
│     // No Argon2id is run during this phase                   │
│     // No password crosses the IPC boundary                   │
└──────────────────────────────────────────────────────────────┘
       │  Frontend displays the decrypted entries
       ▼

Phase 3 — Locking (on demand or auto-lock timer)
═══════════════════════════════════════════════════════════════

Trigger:
  User clicks "Lock" or inactivity timer expires
       │  invoke('session_lock', { sessionId: "a1b2c3d4-..." })
       │  or frontend clears sessionId and calls logout()
       ▼
┌──────────────────────────────────────────────────────────────┐
│ Rust Backend — session::SessionManager::lock(session_id)     │
│                                                               │
│  sessions.remove(session_id)                                  │
│  → The HashMap entry is deleted                               │
│  → The derived_key Vec<u8> is dropped (memory freed)          │
│  → Any subsequent operation with this session_id returns      │
│    Err("未登录或会话已过期")                                  │
│                                                               │
│  Coup de grâce: the cryptographic key is gone from RAM        │
└──────────────────────────────────────────────────────────────┘
       │  Frontend clears sessionId, navigates to LockScreen
       ▼
```

**Why sessions are superior to passing raw passwords:**

| Aspect | Raw Password | Session Key |
|--------|-------------|-------------|
| Frontend memory contains | Plaintext `"bbkb"` | Opaque `"a1b2c3d4-..."` UUID |
| Leakage impact | Complete compromise — password can decrypt everything | Must simultaneously compromise Rust heap + frontend, and session expires |
| Per-operation cost | Argon2id (~300ms, intentional slow hash) | HashMap lookup (~0.001ms) |
| Locking semantics | Simulate by deleting JS variable (client-side only) | Delete key from Rust HashMap (actual cryptographic lock) |
| Password change impact | Must re-encrypt everything with new Argon2id derivation | Same — but only happens once, and old session immediately invalidated |

### Backup & Import Encryption

Backup files use the same encryption as the live vault — they **are** the vault file, just in a different directory. This means "finding a backup file" is not the same as "reading someone's passwords":

```
Backup Flow (export):
  vault.enc ──file copy──→ backup_dir/vault-20260701.enc
  ↑ The file is ALREADY encrypted with AES-256-GCM.
    No additional encryption step needed.

Restore Flow (import from backup directory):
  ┌──────────────────────────────────────────────┐
  │  Requires: USB device authorization only      │
  │  Copies file back from backup_dir             │
  │  File remains encrypted — requires            │
  │  the master password to decrypt               │
  └──────────────────────────────────────────────┘

Import Flow (from arbitrary file):
  ┌──────────────────────────────────────────────┐
  │  1. User selects a .enc file                  │
  │  2. User enters the master password that      │
  │     was used to CREATE that backup            │
  │                                               │
  │  3. Backend: import_from_file(filePath, pwd)  │
  │     a. Read file → extract salt + encrypted   │
  │     b. Derive key = Argon2id(pwd, salt)       │
  │     c. Try AES-256-GCM decryption             │
  │     d. If decryption fails →                  │
  │        "密码错误，无法导入备份文件" ❌         │
  │     e. If decryption succeeds →               │
  │        validate JSON structure                │
  │        copy to ~/.key-vault/vault.enc         │
  │        "导入成功" ✅                          │
  └──────────────────────────────────────────────┘
```

**Security guarantee**: Without the correct master password, an attacker cannot even pass the import gate. The decryption attempt (step 3c) serves as both verification and access control. The file is never written to the vault location unless decryption succeeds.

### Frontend Security Measures

| Measure | Implementation | What It Prevents |
|---------|---------------|------------------|
| Password-free frontend | Only `sessionId` stored after login; password discarded from JS heap | XSS, memory dump, or devtools inspection cannot steal the master password |
| Screenshot protection | `getCurrentWindow().setContentProtected(true)` (Tauri API) | Malware capturing screen contents via PrintScreen, Snipping Tool, or DXGI/DDA APIs |
| IME blocking | CSS `ime-mode: disabled` + `spellcheck="false"` | Chinese/Japanese IME interference causing incorrect password entry |
| Inactivity auto-lock | `setTimeout` per user activity event (mousedown, keydown, touchstart, wheel) | Unattended terminal accessible to others after you step away |
| Activity debounce | Timer resets on every activity event | Accurate idle time measurement without false positives |
| Independent locking | Separate Boolean flags for vault vs. API keys | Granular access control — hide API keys while showing vault entries |

### Threat Model & Attack Surface Analysis

| Attack Vector | Outcome | Reasoning |
|--------------|---------|-----------|
| **Physical USB theft, reading vault.enc** | ❌ Protected | AES-256-GCM with Argon2id-derived key. Without the master password, the ciphertext is computationally indistinguishable from random noise |
| **Master password brute-force** | ❌ Infeasible | Argon2id memory-hardness (64MB memory, 3 iterations): ~300ms per attempt. An 8-character mixed-case alphanumeric password (62^8 ≈ 2.18×10^14) exceeds the age of the universe to exhaust |
| **Quantum computer attack on AES-256** | ❌ Not vulnerable | AES-256's post-quantum security is ~2^128 (Grover's algorithm halves the key space). Still beyond any practical capability |
| **DMA attack reading Rust heap** | ⚠️ Hard | Requires physical PCIe/eSATA access. Windows 10+ includes Kernel DMA Protection by default on modern hardware |
| **Screen capture of decrypted data** | ❌ Blocked | `SetWindowDisplayAffinity(WDA_MONITOR)` prevents any screen capture API from reading the window content |
| **Session ID theft** | ⚠️ Low impact | Session ID alone cannot derive the master password or decrypt data. Attacker would need to simultaneously compromise the running Rust process. Sessions expire on lock |
| **Backup file theft** | ❌ Protected | Import requires the backup's original master password. AES-GCM decryption failure immediately rejects the file |
| **Frontend JS variable inspection** | ❌ Low risk | Only `sessionId` is exposed. No keys or passwords present in JS memory during normal operation |
| **Physical access to unlocked app** | ⚠️ Mitigated | Auto-lock timer returns to lock screen after configurable inactivity period. Default: 5 minutes |
| **Log file containing passwords** | ⚠️ Debug builds only | `tauri_plugin_log` is enabled only in debug builds. Production builds disable logging by default |

---

## 🏗️ Technical Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Tauri v2 Desktop Application                      │
│                                                                     │
│  ┌────────────────────────────┐    ┌────────────────────────────┐  │
│  │   Vue.js 3 Frontend         │    │   Rust Backend             │  │
│  │   (TypeScript)              │    │                            │  │
│  │                              │    │  SessionManager           │  │
│  │   Pinia (state)             │    │  ┌──────────────────┐     │  │
│  │   Vue Router (routing)      │◄──►│  │Mutex<HashMap<    │     │  │
│  │   Naive UI (components)     │ IPC │  │  String, Vec<u8> │     │  │
│  │   Vite (bundler + HMR)      │    │  └──────────────────┘     │  │
│  │                              │    │  crypto.rs               │  │
│  │  Components Layer            │    │  vault.rs / api_keys.rs │  │
│  │   ├─ LockScreen.vue         │    │  backup.rs / auth.rs     │  │
│  │   ├─ VaultView.vue          │    │  config.rs / sha_pin.rs │  │
│  │   ├─ ApiKeysView.vue        │    └────────────────────────────┘  │
│  │   └─ SettingsView.vue       │                                    │
│  └────────────────────────────┘                                    │
└─────────────────────────────────────────────────────────────────────┘
```

**Architecture Decision: Strict Separation of Concerns**

The frontend and backend communicate exclusively through Tauri's IPC mechanism (serialized JSON commands). The cryptographic key **never** crosses this boundary:

| Component | Responsibilities | Runs In | Security Domain |
|-----------|----------------|---------|-----------------|
| Frontend | Rendering, user input, navigation | WebView (Chromium) | Untrusted (by design) |
| Backend | All crypto, file I/O, session mgmt | Native Rust process | Trusted (isolated) |

This design ensures that even if the WebView is compromised (e.g., via a dependency vulnerability), the attacker cannot access cryptographic keys or encrypted data directly — they would need a separate privilege escalation to the Rust process.

### Core Rust Module Reference

| Module | File | Public API | Dependencies | Thread Safety |
|--------|------|-----------|--------------|---------------|
| **session** | `session.rs` | `login(password) → Result<String>` `lock(session_id)` `get_key(session_id) → Option<Vec<u8>>` `change_password(sid, old, new) → Result<String>` `is_active(session_id) → bool` | `sha2`, `uuid`, `std::sync::Mutex` | ✅ `Mutex<HashMap>` |
| **crypto** | `crypto.rs` | `derive_key(password, salt) → Vec<u8>` `encrypt(plaintext, key) → Vec<u8>` `decrypt(data, key) → Result<Vec<u8>>` `generate_salt() → Vec<u8>` | `aes-gcm`, `argon2`, `password-hash`, `rand` | ✅ Pure functions |
| **vault** | `vault.rs` | `load_vault(path, key) → Result<VaultFile>` `save_vault(path, vault, key) → Result<()>` | `serde`, `serde_json` | ✅ Pure functions |
| **api_keys** | `api_keys.rs` | `load_keys(path, key) → Result<Vec<ApiKey>>` `save_keys(keys, path, key) → Result<()>` | `serde`, `serde_json` | ✅ Pure functions |
| **backup** | `backup.rs` | `backup_vault(vault_path, backup_root) → Result<String>` `restore_vault(target, backup_root, filename) → Result<()>` `list_backups(backup_root) → Result<Vec<String>>` | `chrono`, `std::fs` | ✅ Stateless |
| **auth** | `auth.rs` | `is_authorized() → bool` `generate_key() → Result<()>` `remove_auth() → Result<()>` (+ `_with_path` variants) | `rand`, `hex` | ✅ Stateless |
| **config** | `config.rs` | `load_config(path) → AppConfig` `save_config(path, config) → Result<()>` | `serde`, `serde_json` | ✅ Stateless |
| **sha_pin** | `sha_pin.rs` | `compute_with_len(input1, input2, len) → Result<(String,String,String)>` | `sha2` | ✅ Pure functions |

### Complete Directory Reference

```
kay-vault/
│
├── src/                          ← Vue.js 3 Frontend (TypeScript)
│   ├── App.vue                   ← Root component: lock screen, sidebar,
│   │                                stats panel, screenshot protection,
│   │                                auto-lock timer
│   ├── main.ts                   ← Vue application entry point
│   ├── env.d.ts                  ← TypeScript ambient declarations
│   │
│   ├── router/
│   │   └── index.ts              ← Route definitions: vault, api-keys,
│   │                                terminal, settings
│   │
│   ├── stores/                   ← Pinia state management
│   │   ├── app.ts                ← Application state: sessionId,
│   │   │                            unlock/lock flags, autoLockMinutes,
│   │   │                            login()/logout() methods
│   │   └── vault.ts              ← Vault data: entries array,
│   │                                search/filter, tree structure,
│   │                                CRUD operations
│   │
│   ├── views/                    ← Route-level page components
│   │   ├── VaultView.vue         ← Password vault: tree navigation
│   │   │                            (group → category → entries),
│   │   │                            card list with copy/search
│   │   ├── ApiKeysView.vue       ← API key management: provider
│   │   │                            grouping, inline editing,
│   │   │                            key masking/reveal, copy
│   │   ├── TerminalView.vue      ← SHA-PIN terminal: prompt-based
│   │   │                            interaction, result display
│   │   └── SettingsView.vue      ← Settings: auto-lock duration,
│   │                                backup path selection (native
│   │                                folder picker), password change,
│   │                                USB authentication, backup panel
│   │
│   ├── components/               ← Reusable UI components
│   │   ├── LockScreen.vue        ← Master password entry + warning
│   │   ├── PasswordForm.vue      ← Entry creation/editing form
│   │   ├── PasswordCard.vue      ← Entry display card
│   │   ├── PasswordGenerator.vue ← Random password generator
│   │   ├── BackupPanel.vue       ← Backup/Restore/Import UI
│   │   └── DeviceAuthList.vue    ← USB authentication management
│   │
│   └── styles/
│       └── theme.css             ← CSS custom properties (colors,
│                                    fonts, border radii, spacing)
│
├── src-tauri/                    ← Rust Backend
│   ├── src/
│   │   ├── main.rs               ← Tauri entry point (cfg_attr mobile)
│   │   ├── lib.rs                ← Tauri command registration,
│   │   │                            path management, Builder setup,
│   │   │                            disk statistics, URL opener,
│   │   │                            all #[tauri::command] functions
│   │   ├── session.rs            ← SessionManager: login verification
│   │   │                            (Argon2id + SHA-256), key caching
│   │   │                            (HashMap), lock/change_password
│   │   ├── crypto.rs             ← Cryptographic primitives
│   │   ├── vault.rs              ← Vault file I/O + structs
│   │   ├── api_keys.rs           ← API key file I/O + structs
│   │   ├── auth.rs               ← USB device authentication
│   │   ├── backup.rs             ← Backup/restore directory ops
│   │   ├── config.rs             ← AppConfig struct + JSON I/O
│   │   └── sha_pin.rs            ← SHA-PIN algorithm implementation
│   │
│   ├── Cargo.toml                ← Rust dependencies
│   ├── tauri.conf.json           ← Tauri build/window/security config
│   ├── build.rs                  ← Tauri build script
│   └── capabilities/
│       └── default.json          ← Tauri v2 capability permissions
│
├── docs/
│   └── encryption-design.md      ← Full encryption architecture doc
│
├── 启动-调试程序.bat              ← Windows dev launcher script
├── package.json                  ← Node.js dependencies & scripts
├── vite.config.ts                ← Vite bundler configuration
├── tsconfig.json                 ← TypeScript config (root)
├── tsconfig.app.json             ← TypeScript config (app code)
├── tsconfig.node.json            ← TypeScript config (build scripts)
└── .gitignore                    ← Git exclusion rules
```

### Technology Stack

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| Frontend Framework | Vue.js 3 | ^3.5 | Reactive UI with Composition API |
| Build Tool | Vite | ^8.1 | Dev server with HMR, production bundling |
| Type System | TypeScript | ~6.0 | Static type checking |
| UI Component Library | Naive UI | ^2.44 | Dark-theme Vue 3 components |
| State Management | Pinia | ^3.0 | Type-safe store |
| Routing | Vue Router | ^4.6 | SPA navigation |
| Desktop Framework | Tauri v2 | ^2.11 | Native window, IPC, system integration |
| Backend Language | Rust | 2021 edition | Memory-safe systems programming |
| Block Cipher | AES-256-GCM | 0.10 | Authenticated encryption (NIST SP 800-38D) |
| Key Derivation | Argon2 | 0.5 | Memory-hard password hashing |
| Hashing | SHA-2 | 0.10 | Verification tag generation |
| Serialization | Serde JSON | 1.0 | Structured data I/O |
| UUID | uuid | 1.0 (v4) | Session identifier generation |

---

## 🚀 Quick Start

### Prerequisites

| Dependency | Minimum Version | Purpose |
|-----------|----------------|---------|
| Windows | 10+ | Target platform (macOS/Linux support planned) |
| Node.js | 18+ | Frontend build toolchain |
| Rust | 1.77+ | Backend compilation |
| System Libraries | — | [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/) |

### Build from Source

```bash
# 1. Clone the repository
git clone https://github.com/O-My-PCCCCCCCCCC/kay-vault.git
cd kay-vault

# 2. Install frontend dependencies
npm install

# 3. Start development mode (Vite + Tauri with hot reload)
npm run tauri dev
# This will:
#   - Start the Vite dev server at http://localhost:5173
#   - Compile the Rust backend (first build: ~2-3 minutes)
#   - Open a native Tauri window

# 4. Production build
npm run tauri build
# Output: src-tauri/target/release/bundle/
```

### Windows Debug Launcher

Double-click **`启动-调试程序.bat`** in the project root:

```
============================================
  * 凯伊密码管家 - 开发调试模式
============================================

[1/3] 清理残留进程...       ← Auto-kills old Vite + app windows
[2/3] 检查依赖...           ← Checks node_modules, auto-installs
[3/3] 启动 Tauri...

  前端端口: http://localhost:5173

→ Press Ctrl+C to stop debugging
→ Press any key to close window
```

### Development Commands

```bash
npm run dev          # Start Vite frontend only (no Tauri window)
npm run build        # TypeScript type check + frontend production build
npm run preview      # Preview production build output
npm run type-check   # TypeScript type check only (no build)
npm run tauri dev    # Full Tauri dev mode (frontend + backend)
npm run tauri build  # Production build (installer package)
```

### Branch Strategy

| Branch | Purpose | Status |
|--------|---------|--------|
| `main` | Stable releases, production builds | ✅ Active |
| `feat/terminal-and-api` | Active development: SHA-PIN terminal, API key management | 🚧 In development |
| `master` | Legacy default (from original repo), synced with `main` | 📦 Archived |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

*Created by [追寻光的影](https://github.com/O-My-PCCCCCCCCCC) · 2026*
