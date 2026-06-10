# AIOS

> AI is not an app. AI is the operating-system interface.

AIOS is an AI-native operating system architecture where users express intent,
and the system invokes native applications, tools, policies, and confirmations
around that intent.

<p align="center">
  <img src="docs/assets/aios-homepage-preview.svg" alt="AIOS homepage preview" width="100%" />
</p>

```text
User intent
-> AIOS Core
-> Policy Kernel
-> Native App Invocation
-> User Confirmation
-> Audited Execution
```

AIOS is an AI-native operating system architecture.

It is not an assistant app. It treats goals, tasks, agents, tools, evidence,
policies, artifacts, and context as system-level objects.

## Why This Matters

Today, users operate applications manually:

```text
Open app -> find contact -> type message -> click send
```

AIOS turns this into an operating-system flow:

```text
"Tell Zhang San dinner is waxiang chicken"
-> invoke chat app
-> show message preview
-> confirm
-> send
```

The user does not start from apps. The user starts from intent.

## Current Status

This repository is an early project scaffold. It contains:

- architecture documents
- Tool ABI and object schemas
- a static UI mock kept only as a visual reference
- a minimal Rust workspace for the trusted core
- native platform and shell abstractions

## Homepage Concept

The homepage is not a website and not a chatbot.

It is a native intent surface:

- AIOS Core stays at the center.
- Native applications appear only when invoked.
- Policy, execution, and audit are visible as system context.
- The user sees the final action preview before execution.

## Core Thesis

Traditional operating systems are application-first:

```text
User -> App -> Process -> Syscall -> Kernel -> Hardware
```

AIOS is goal-first:

```text
User -> Goal -> Agent Process -> Tool Call -> AI Kernel -> Resource / Device / API
```

In AIOS:

- Agent is a system process.
- Tool is a controlled system capability.
- Goal is the primary user entry.
- Evidence is a first-class execution record.
- Policy is the boundary for agent behavior.
- Shell is adaptive across phone, desktop, tablet, car, and embedded devices.

## Architecture

```text
AIOS
├─ kernel/              AI-native kernel modules
├─ runtime/             Agent, workflow, tool, and model runtimes
├─ shells/              Adaptive user shells for different devices
├─ specs/               System protocols and object schemas
├─ services/            System services built on top of the core
├─ plugins/             First-party and third-party capability adapters
├─ examples/            Reference scenarios and task flows
└─ docs/                Product, architecture, and roadmap documents
```

## Key Documents

- [Architecture](docs/ARCHITECTURE.md)
- [Project Structure](docs/PROJECT_STRUCTURE.md)
- [Technology Stack](docs/TECH_STACK.md)
- [Final Project Structure](docs/FINAL_PROJECT_STRUCTURE.md)
- [Core Objects](docs/CORE_OBJECTS.md)
- [Adaptive Shell](docs/ADAPTIVE_SHELL.md)
- [Homepage Effect](docs/HOMEPAGE_EFFECT.md)
- [UI Reference](docs/UI_REFERENCE.md)
- [MVP Roadmap](docs/MVP_ROADMAP.md)

## First Build Target

The first version should not try to replace every part of Windows, Linux, or
mobile operating systems.

The first version should prove the AIOS loop:

```text
Goal -> Plan -> Agent Runtime -> Tool Call -> Verification -> Artifact -> Audit
```

Once that loop is stable, AIOS can expand from an execution layer into a fuller
device-native system.

## Quick Start

Run the native core checks:

```bash
cargo test --workspace
```

The HTML files under `prototypes/shell-ui` are design references only. They are
not the product architecture and should not be treated as the final shell
technology.

The intended product line is native:

```text
Rust core
+ native platform adapters
+ native shell renderer
+ WASM plugin sandbox
```

## GitHub Upload

Initialize and push:

```bash
git init
git add .
git commit -m "Initial AIOS scaffold"
git branch -M main
git remote add origin https://github.com/<your-org>/<your-repo>.git
git push -u origin main
```
