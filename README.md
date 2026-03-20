# DevHosts

DevHosts is a Tauri v2 desktop app scaffold with a SolidJS + Vite frontend and a Rust backend.

Current state:
- Legacy C# project artifacts removed.
- Rust module architecture scaffolded for continued implementation.
- System command stubs wired through Tauri `invoke`.

## Prerequisites

- Node.js 20+ and npm
- Rust toolchain (`rustup`, `cargo`, `rustc`)
- Windows: WebView2 runtime and MSVC build tools

## Setup

```bash
npm install
```

## Run in dev mode

```bash
npm run tauri dev
```

Frontend only:

```bash
npm run dev
```

## Build desktop app

```bash
npm run tauri build
```

## Project layout

```text
src/
  App.tsx
  main.tsx

src-tauri/
  src/
    core/
      error.rs
      response.rs
    commands/
      system.rs
    modules/
      system/
        hosts.rs
        ports.rs
      hash/
      crypto/
      encoding/
      format/
      convert/
```

## Implemented command stubs

- `list_hosts`
- `add_host`
- `remove_host`
- `clean_hosts`
- `list_ports`
- `kill_port`

All currently return compile-safe placeholder responses with no host/process mutation yet.
