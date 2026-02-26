# 🚀 FeatureFlags.rs - Development Roadmap

## 🎯 **Vision**
Ultra-simple, offline-first feature flag manager for indie devs & small teams. **50MB Docker → production in 2 minutes**. No SaaS, no Kubernetes, no $100+/month subscriptions.

***

## 🛤️ **Version Roadmap**

## **v0.1.0 - MVP** ✅ *Current*
**"Works on my machine → works in prod"**

| Feature | Status | Deliverable |
|---------|--------|-------------|
| Axum server (3 endpoints) | ✅ Done | `GET /flags`, `GET /evaluate`, `POST /flags/{name}` |
| YAML persistence | ✅ Done | `./flags/flags.yml` |
| 3 rule types | ✅ Done | Global, `% rollout`, `attr=val` |
| Python SDK (PyO3) | ✅ Done | `client.is_enabled()` |
| CLI (`ffctl`) | ✅ Done | `create`, `list`, `toggle` |
| Docker image | ✅ Done | `< 50MB` (Alpine, ~17MB) |

**Success:** `docker run → curl → Python app works`

**v0.1.0 verification:**
- [x] `cargo test --workspace` passes (core evaluation tests)
- [x] Server: `GET /flags`, `GET /evaluate`, `POST /flags/{name}` with YAML at `FLAGS_PATH`
- [x] CLI: `ffctl create/list/enable/disable` and `ffctl rule add`
- [x] Python: `FeatureClient(server_url).is_enabled(name, user_id=..., attributes=...)` (pip install -e apps/client-py)
- [x] Docker: `docker build -f docker/Dockerfile -t featureflags/server .` → image < 50MB (~17MB Alpine)

***

## **v0.2.0 - Usable**
**"My team can use this daily"**

```
[ ] SQLite storage (migrations)
[ ] Basic HTML dashboard (no JS)
[ ] `ffctl export/import`
[ ] Config file (`featureflags.toml`)
[ ] Healthcheck endpoint
[ ] Graceful shutdown
[ ] 90% test coverage
```

***

## **v0.3.0 - Production**
**"Rust-native + Variants support"**

```
[ ] Rust SDK crate (publish crates.io)
[ ] Flag variants (A/B, multivariate)
[ ] Percentage rollout (`hash(user_id) % 100`)
[ ] TTL cache in SDKs
[ ] `ffctl rollout 10% new_feature`
[ ] Metrics endpoint (`/metrics`)
```

***

## **v0.4.0 - Secure**
**"Safe for small teams"**

```
[ ] API token auth (Bearer)
[ ] `ffctl token create/read`
[ ] Rate limiting
[ ] Input validation (SQLi, etc.)
[ ] Audit log (who changed what)
[ ] HTTPS support (certificates)
[ ] Environment variables config
```

***

## **v1.0.0 - Production Ready**
**"Nobody calls it beta anymore"**

```
[ ] Full docs (mdbook)
[ ] CHANGELOG.md automated
[ ] GitHub Actions (CI/CD)
[ ] Docker Hub publish
[ ] `cargo install ffctl`
[ ] `pip install featureflags-client`
[ ] Benchmarks (500 req/s)
[ ] Load testing (1000 concurrent)
```

**Success Metrics:**
```
✅ Docker pulls: 100+/week
✅ Crates.io: 50+ downloads/week
✅ Stars: 100+ on GitHub
✅ Zero critical CVEs
```

***

## **v1.1.0 - Polished**
**"Joy to use"**

```
[ ] Web dashboard (HTMX + Tailwind)
[ ] Bulk operations (`ffctl bulk-edit`)
[ ] Backup/restore
[ ] Git sync (`flags → git push`)
[ ] Slack/Discord notifications
[ ] VSCode extension
```

***

## 🔮 **Future (v2.0+)**
```
[ ] Multi-environment (dev/staging/prod)
[ ] Rollback strategies
[ ] A/B testing analytics
[ ] OpenTelemetry
[ ] Kubernetes operator
[ ] WASM edge runtime
```

***

## 📈 **Success Metrics by Version**

| Version | Docker Size | RAM | Resp Time | Stars | Downloads |
|---------|-------------|-----|-----------|-------|-----------|
| v0.1.0  | <50MB       | 50MB| 5ms      | 10    | -         |
| v0.2.0  | <60MB       | 75MB| 3ms      | 50    | -         |
| v0.3.0  | <70MB       | 100MB| 2ms     | 100   | 100/wk    |
| v1.0.0  | <80MB       | 150MB| 1ms     | 250   | 500/wk    |
| v1.1.0  | <90MB       | 200MB| 1ms     | 500   | 1K/wk     |

***

## 🎮 **Current Status: v0.1.0 READY**

```bash
# 30-second demo (works NOW)
cargo run -p featureflags-server   # or: cargo run --bin featureflags-server
cargo run -p featureflags-cli -- create new_checkout --description "New checkout"
curl "http://localhost:8080/evaluate?flag=new_checkout&user_id=123"

# Docker
docker build -f docker/Dockerfile -t featureflags/server .
docker run -p 8080:8080 -v ./flags:/data featureflags/server
```

**Workspace binaries:** Use `-p` or `--bin` when running from repo root (no default binary):
- `cargo run -p featureflags-server` — server
- `cargo run -p featureflags-cli -- <args>` — ffctl (e.g. `create`, `list`, `enable`)

***

## 📋 **Version Details & Priorities**

### v0.2.0 – Suggested order
1. **SQLite storage** — Replace/supplement YAML; migrations for schema.
2. **Healthcheck** — `GET /health` (and optional readiness).
3. **Config file** — `featureflags.toml` (server URL for CLI, port, data path).
4. **Basic HTML dashboard** — Server-served HTML, no JS; list flags, toggle.
5. **Graceful shutdown** — SIGTERM → drain, then exit.
6. **Export/import** — `ffctl export > backup.yml`, `ffctl import < backup.yml`.
7. **Test coverage** — Aim 90% (unit + integration).

### v0.3.0 – Rust SDK
- Publish `featureflags-client` (or `featureflags-sdk`) on crates.io.
- Same evaluation logic as server; optional HTTP fallback.
- Variants and percentage rollout already in core; expose in API and SDK.

### v1.0.0 – Release checklist
- [ ] Semantic versioning; tagged releases.
- [ ] CHANGELOG.md (keep-a-changelog style).
- [ ] CI (GitHub Actions): `cargo test`, `cargo clippy`, `cargo fmt -- --check`.
- [ ] Publish: `cargo install ffctl` / `featureflags-server`, `pip install featureflags-client`.
- [ ] Docker Hub (or GHCR) image; documented run command.
- [ ] Basic benchmarks and load targets documented.

***

## 🔄 **Release & Versioning**

- **Versioning:** Semantic versioning (MAJOR.MINOR.PATCH).
- **Branching:** `main` = stable; feature branches → PR.
- **Releases:** Tag `v0.2.0`, `v1.0.0`; attach binaries/wheels (optional).
- **Changelog:** Update `CHANGELOG.md` (or roadmap) per release.

***

## ⚠️ **Out of scope (v1)**

Per README, we explicitly **do not** plan for v1:
- Complex UI/analytics, A/B stats, RBAC, webhooks.
- Kubernetes operator, Redis/Postgres, multi-tenant.
- Multiple SDKs beyond Python + Rust.

Revisit in v2+ if the project grows.

***

**🚀 Next Milestone: v0.2.0 (SQLite + Dashboard)**

**Your move:** Pick 1-2 features from v0.2.0 and we'll code them next! 🦀