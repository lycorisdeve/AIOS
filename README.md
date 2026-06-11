<p align="center">
  <img src="docs/assets/aios-logo.svg" alt="AIOS logo" width="120" />
</p>

<h1 align="center">AIOS</h1>

<p align="center">
  <strong>AI is not an app. AI is the operating-system interface.</strong>
</p>

<p align="center">
  <a href="README.md">English</a>
  ·
  <a href="README.zh-CN.md">简体中文</a>
  ·
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <img alt="CI" src="https://img.shields.io/github/actions/workflow/status/lycorisdeve/AIOS/ci.yml?branch=master&label=CI" />
  <img alt="License" src="https://img.shields.io/github/license/lycorisdeve/AIOS" />
  <img alt="Rust" src="https://img.shields.io/badge/core-Rust-orange" />
  <img alt="Status" src="https://img.shields.io/badge/status-early%20prototype-blue" />
  <img alt="PRs" src="https://img.shields.io/badge/PRs-welcome-brightgreen" />
</p>

<p align="center">
  <img src="docs/assets/aios-homepage-preview.svg" alt="AIOS homepage concept preview" width="100%" />
</p>

## What Is AIOS?

AIOS is an experimental, AI-native operating-system architecture.

It is not a chatbot, not a web frontend, and not an Electron-style shell. AIOS
explores a different system model:

```text
User intent
-> AIOS Core
-> Policy Kernel
-> Native App Invocation
-> User Confirmation
-> Audited Execution
```

Users should not have to begin by opening apps. Users should express intent,
and the operating system should invoke the right native application, tool,
policy, preview, and confirmation flow around that intent.

Example:

```text
Tell Zhang San: dinner is waxiang chicken tonight.
```

Traditional OS flow:

```text
Open chat app -> find Zhang San -> type message -> verify -> send
```

AIOS flow:

```text
Understand intent
-> invoke the chat app
-> show a native message preview
-> wait for confirmation
-> send
-> audit the action
```

## Why It Matters

Most operating systems are still application-first:

```text
User -> App -> Window -> Button -> Result
```

AIOS is intent-first:

```text
User -> Intent -> AIOS Core -> Native App / Tool -> Confirmation -> Result
```

That changes the role of the interface:

- AI is the system interface, not another app.
- Apps still exist, but they are invoked by intent.
- Agents become managed system processes.
- Tools become controlled system capabilities.
- Policies define what AI can and cannot do.
- Evidence and audit logs become part of system trust.

## Design Principles

1. **Intent first**  
   The user describes the desired outcome. AIOS decides which native app,
   system tool, model, or workflow is needed.

2. **Native first**  
   The final shell should be native. HTML prototypes are visual references, not
   the product runtime.

3. **Policy first**  
   AI should not execute sensitive actions freely. Permissions, risk, approval,
   and audit are core OS responsibilities.

4. **Apps as capabilities**  
   Chat, calendar, files, browser, mail, terminal, and enterprise systems should
   be callable capabilities, not mandatory starting points.

5. **Cross-device by design**  
   Phone, desktop, tablet, car, and embedded devices should share one AIOS Core
   with adaptive native shells.

## Architecture

```text
AIOS
├─ crates/
│  ├─ aios-types              shared system object types
│  ├─ aios-goal-kernel        intent and goal decomposition
│  ├─ aios-policy-kernel      permission, risk, approval decisions
│  ├─ aios-agent-runtime      agents as managed system processes
│  ├─ aios-platform           native platform abstraction
│  ├─ aios-native-shell       native shell state model
│  ├─ aios-daemon             local daemon prototype
│  └─ aios-cli                command-line prototype
├─ specs/
│  ├─ objects                 Goal, AgentProcess, Artifact schemas
│  ├─ policy                  policy decision contracts
│  ├─ tool-abi                controlled tool capability ABI
│  └─ workflow                workflow state contracts
├─ docs/                      architecture and roadmap
├─ prototypes/                visual references only
├─ plugins/                   future tool/plugin adapters
├─ services/                  system service boundaries
├─ examples/                  scenario references
└─ tests/                     contract, policy, runtime, shell tests
```

## Technology Direction

Primary path:

```text
Rust       -> AIOS Core, kernel modules, runtime, native shell model
WASM       -> plugin sandbox and portable controlled tools
C / C++    -> low-level platform and driver interop when unavoidable
Python     -> model experiments and evaluation outside the trusted core
```

Not the product path:

```text
Node       -> not required by the core
Electron   -> not the target shell
HTML UI    -> visual reference only
Web backend-> not the core architecture
```

## Current Status

AIOS is very early. The repository currently provides:

- Rust workspace scaffold
- core system object types
- Goal Kernel prototype
- Policy Kernel prototype
- Agent Runtime prototype
- Native Shell state model
- Native Platform abstraction
- Tool ABI and workflow schemas
- homepage concept preview
- architecture, roadmap, and design documents

## Quick Start

Install Rust, then run:

```bash
cargo test --workspace
```

Run the CLI prototype:

```bash
cargo run -p aios-cli -- "Tell Zhang San dinner is waxiang chicken tonight"
```

Run the daemon prototype:

```bash
cargo run -p aios-daemon
```

## Roadmap

### Phase 0: Concept Scaffold

- [x] Define the AIOS thesis
- [x] Create Rust workspace
- [x] Add core object types
- [x] Add Goal Kernel prototype
- [x] Add Policy Kernel prototype
- [x] Add Native Shell model
- [x] Add visual homepage concept

### Phase 1: Native Core MVP

- [ ] Expand Goal / Task / Plan models
- [ ] Implement a minimal workflow runtime
- [ ] Add Tool ABI registration and invocation
- [ ] Add audit record generation
- [ ] Add policy-backed confirmation flow

### Phase 2: Intent Invocation Demo

- [ ] Parse a natural-language intent
- [ ] Select a native app target
- [ ] Show an app preview surface
- [ ] Ask for user confirmation
- [ ] Execute through a controlled capability
- [ ] Record an audit trail

### Phase 3: Native Shell Experiments

- [ ] Windows native shell experiment
- [ ] Linux native shell experiment
- [ ] macOS native shell experiment
- [ ] Cross-device shell state model

## Contributing

AIOS is meant to be co-created.

Good first contribution areas:

- kernel and runtime design
- native shell model
- platform adapters
- policy and security model
- Tool ABI and plugin sandbox
- real-world intent scenarios
- architecture documentation
- visual system design

Open an issue with a scenario, question, design sketch, or implementation idea.
Pull requests are welcome.

## Key Documents

- [Architecture](docs/ARCHITECTURE.md)
- [Native OS Strategy](docs/NATIVE_OS_STRATEGY.md)
- [Technology Stack](docs/TECH_STACK.md)
- [Core Objects](docs/CORE_OBJECTS.md)
- [Final Project Structure](docs/FINAL_PROJECT_STRUCTURE.md)
- [Adaptive Shell](docs/ADAPTIVE_SHELL.md)
- [Homepage Effect](docs/HOMEPAGE_EFFECT.md)
- [MVP Roadmap](docs/MVP_ROADMAP.md)

## License

MIT License.

