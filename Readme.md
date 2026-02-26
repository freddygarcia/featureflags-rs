# 🚀 FeatureFlags v1 - Ultra-Simple Offline Feature Flag Manager

## 🎯 Project Goals (MVP Scope - 1 Week)

**WHAT WE BUILD:**
- Single HTTP server (~100 LOC) that serves feature flags
- File-based persistence (`flags.yml`)
- 3 simple rules: global on/off, % rollout, attribute equality
- Python SDK with local evaluation + cache
- Minimal CLI (`ffctl`)
- Docker image < 50MB

**WHAT WE DON'T BUILD (v1):**
- No complex UI, no A/B stats, no RBAC, no webhooks
- No Kubernetes, no Redis, no multi-DB

**SUCCESS =** Clone → docker up → create flag → use in Python app (2 minutes total)

---


**Simple, offline-first feature flags for indie devs and small teams.** No SaaS, no Kubernetes, no complexity. Just flags that work.

## 🚀 Quick Start (2 minutes)

```bash
# 1. Clone & build
git clone <repo>
cd featureflags
cargo build

# 2. Run server (persists to ./flags/flags.yml)
cargo run -p featureflags-server

# 3. Create flag
cargo run -p featureflags-cli -- create new_checkout --description "New checkout flow"

# 4. Use in Python via PyO3
pip install -e apps/client-py
```

```python
# Python app
from featureflags_client import FeatureClient

client = FeatureClient("http://localhost:8080")
if client.is_enabled("new_checkout", user_id="123", attributes={"country": "ES"}):
    show_new_checkout()
```

## 🎯 What Makes It Different

| ❌ Others (Flagsmith, Unleash) | ✅ FeatureFlags.rs |
|-------------------------------|-------------------|
| Enterprise platform (500MB+) | Single 50MB Docker image |
| Complex UI + analytics | CLI-first, optional HTML |
| Redis/Kafka required | File-based (YAML/SQLite) |
| 15+ SDKs | Python + Rust (PyO3) |
| Multi-tenant RBAC | Single team, token auth |

**Perfect for:** Indie hackers, side projects, micro-SaaS, small teams (<10 devs)

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   ffctl CLI     │───▶│  Server      │───▶│ Python/Rust SDK │
│  (clap)         │    │ (Axum)       │    │  (PyO3)         │
└─────────────────┘    │              │    └─────────────────┘
                       │  flags.yml   │
                       └──────────────┘
```

**Single source of truth:** `flags.yml` file. Edit manually or via CLI.

## 📋 Supported Features (v0.1)

### ✅ **Included**
- `GET /flags` - List all flags
- `GET /evaluate` - Evaluate flag for user
- `POST /flags/{name}` - Create/update flags
- **3 rule types:** Global on/off, % rollout, attribute equality
- YAML persistence ( `./flags/flags.yml`)
- Python SDK (local evaluation + cache)
- Professional CLI (`ffctl create/list/toggle`)

### ❌ **Not in v1** (keeps it lean)
- No complex UI/dashboard
- No A/B testing analytics
- No multi-tenant/users
- No WebSockets/realtime
- No Redis/Postgres
- No Kubernetes operator

## 💾 Example `flags.yml`

```yaml
flags:
  new_checkout:
    description: "New checkout flow"
    enabled: true
    rules:
      - attribute: country
        operator: eq
        value: "ES"
        enabled: true
      - attribute: user_id
        operator: percent
        value: 10
        enabled: true
  dark_mode:
    description: "Dark theme"
    enabled: false
```

## 🛠️ Tech Stack

```
Core: Rust 1.75+ | Serde | thiserror
Server: Axum | Tokio
CLI: Clap v4
Python: PyO3 | maturin
Persist: YAML (v1) | SQLite (v2)
```

## 📦 Installation

### Option 1: Cargo Install (Recommended)
```bash
cargo install ffctl  # CLI only
cargo install featureflags-server  # Server only
```

### Option 2: Docker
```bash
docker build -f docker/Dockerfile -t featureflags/server .
docker run -p 8080:8080 -v ./flags:/data featureflags/server
```
(Server reads/writes `FLAGS_PATH`; default in image is `/data/flags.yml`.)

### Option 3: Full Workspace
```bash
git clone https://github.com/YOURORG/featureflags-rs
cd featureflags
cargo build --release
```

## 🎮 CLI Usage

```bash
# Create flag
ffctl create new_checkout --description "New checkout"

# Add rule
ffctl rule add new_checkout --country ES --enabled true

# List flags
ffctl list

# Toggle
ffctl enable new_checkout
ffctl disable dark_mode
```

## 🧪 Example Usage

**1. Start server:**
```bash
cargo run -p featureflags-server
```

**2. Create flag:**
```bash
cargo run -p featureflags-cli -- create premium_features --description "Premium UI"
```

**3. Python app:**
```python
from featureflags_client import FeatureClient

client = FeatureClient("http://localhost:8080")
if client.is_enabled("premium_features", "user123", {"plan": "pro"}):
    show_premium_dashboard()
```

## 📁 Workspace Structure

```
featureflags/
├── apps/
│   ├── server/     # Axum HTTP server
│   ├── cli/        # ffctl binary
│   └── client-py/  # PyO3 Python bindings
├── libs/
│   └── core/       # Shared Flag/Rule models
├── flags/          # Persisted data
└── docker/
```

## 🚀 Roadmap

```
v0.1 [THIS]  Ultra-simple server + Python SDK
v0.2         SQLite + basic HTML dashboard
v0.3         Rust SDK crate
v1.0         Variants + percentage rollout
```

## 🤝 Contributing

```bash
# Setup dev
cargo build
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Run examples
cargo run -p apps::server
cargo run -p apps::cli -- create test
```

1. Fork → PR to `main`
2. `cargo fmt` + `cargo clippy`
3. Add tests for new features

## 📄 License

MIT © 2026. Use anywhere, modify freely.

## 💰 Why Build This?

**Problem:** LaunchDarkly/Flagsmith = $100+/month + infra headaches
**Solution:** 50MB Docker image, $0/month, works offline

**Built for indie hackers who want flags without the enterprise bloat.**

***

⭐ **Star if you find it useful!**
🐛 **Issues/PRs welcome**
💬 **@your_twitter** | **discord.gg/rust**

***

```bash
# Try it now (30 seconds)
mkdir -p flags
cargo run -p featureflags-server   # in one terminal
curl http://localhost:8080/flags   # in another
```