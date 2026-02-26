# MegaDB Web

Rust + WASM Web Client for [MegaDB](https://github.com/younjin-jeong/MegaDB) — a Kubernetes-native hybrid database engine for cloud cost intelligence.

## Features

- **SQL Editor** — CodeMirror 6 with SQL syntax highlighting, multi-tab queries, Ctrl+Enter execution
- **Schema Browser** — Database/schema/table tree with column details, partition info, storage engine type
- **K8s Dashboard** — Real-time pod status, scaling controls, disk/partition management
- **Performance Monitoring** — Query throughput, latency percentiles, active queries, resource usage

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | [Leptos](https://leptos.dev/) (Rust + WASM) |
| SQL Editor | CodeMirror 6 |
| Server | Axum (SSR + API proxy) |
| Charts | Charming (Apache ECharts) — Phase 4 |
| Real-time | WebSocket |
| Local Storage | IndexedDB |

## Architecture

```
Browser (WASM)  ←→  Leptos Server (SSR + API Proxy)  ←→  MegaDB HTTP :8080
                                                      ←→  K8s API
                                                      ←→  Prometheus :9090
```

The web client is 100% independent of MegaDB core. It communicates only through MegaDB's public HTTP API and can be removed without any impact.

## Quick Start

```bash
# Prerequisites
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown

# Development
cargo check --workspace
cargo test --workspace

# With cargo-leptos (full dev server)
cargo install cargo-leptos
cargo leptos watch
```

## Project Structure

```
crates/
├── megaweb-types/    ← Shared types (query, schema, k8s, metrics)
├── megaweb-app/      ← Leptos UI (pages, components, state)
└── megaweb-server/   ← Axum server (SSR, API proxy, WebSocket)
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
