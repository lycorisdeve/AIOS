# Technology Stack

AIOS should use a layered language strategy.

The rule is simple:

```text
Core safety and scheduling: Rust
Adaptive shell and developer surface: TypeScript
Model and research workflows: Python
Plugin isolation: WebAssembly
Schemas and contracts: JSON Schema / Protobuf
Driver and platform interop: C / C++ only where necessary
```

## Language Choices

### Rust

Rust is the primary system language.

Use Rust for:

- Goal Kernel
- Policy Kernel
- Context Core
- Agent Runtime
- Execution Runtime
- Tool Runtime
- Model Runtime
- Resource Scheduler
- Artifact Service
- Audit Service
- Identity Service
- Sync Service
- native CLI
- local daemon

Why:

- memory safety
- predictable performance
- strong concurrency
- good long-running service behavior
- suitable for permission and sandbox boundaries

### TypeScript

TypeScript is not part of the native OS core.

It may be used only for optional design tooling, developer dashboards, or SDKs
that sit outside the trusted system path.

Do not use TypeScript, Electron, or a browser WebView as the final OS shell.

### Python

Python is not part of the trusted core.

Use Python for:

- model experiments
- evaluation scripts
- prompt tests
- dataset preparation
- research notebooks
- optional model adapters
- offline analysis jobs

Python agents can exist, but they must run inside controlled runtime boundaries.

### WebAssembly

WebAssembly is the plugin sandbox target.

Use WASM for:

- third-party plugins
- enterprise capability adapters
- deterministic tool extensions
- portable local tools

Plugins should communicate through Tool ABI, not direct system access.

### C / C++

C and C++ should be limited.

Use only for:

- existing native library interop
- hardware/driver bindings
- platform APIs not exposed cleanly to Rust

Do not use C/C++ for policy, identity, audit, or tool execution control unless
there is no practical alternative.

## System Boundary

```text
Trusted Core:
  Rust

User Shell:
  TypeScript

Research and Model Tooling:
  Python

Plugin Sandbox:
  WASM

Interop Edge:
  C / C++
```

## First Implementation Recommendation

Build the first usable version as:

```text
Rust core daemon
+ Rust native shell model
+ native platform adapters
+ WASM tool plugins
+ Python model adapters outside the trusted core
+ JSON/Protobuf contracts
```

This keeps the system serious enough to be an OS architecture, while still
allowing fast iteration on UI and AI behavior.
