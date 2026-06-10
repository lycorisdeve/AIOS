# Project Structure

This repository is organized as an AI-native OS project, not a single app.

```text
AIOS
├─ kernel/
│  ├─ goal-kernel/
│  ├─ policy-kernel/
│  └─ context-core/
├─ runtime/
│  ├─ agent-runtime/
│  ├─ execution-runtime/
│  ├─ model-runtime/
│  └─ tool-runtime/
├─ shells/
│  ├─ desktop-shell/
│  ├─ mobile-shell/
│  ├─ tablet-shell/
│  ├─ car-shell/
│  └─ shared-shell-kit/
├─ specs/
│  ├─ objects/
│  ├─ tool-abi/
│  ├─ policy/
│  └─ workflow/
├─ services/
│  ├─ artifact-service/
│  ├─ audit-service/
│  ├─ identity-service/
│  └─ sync-service/
├─ plugins/
│  ├─ file-system/
│  ├─ browser/
│  ├─ email/
│  ├─ database/
│  └─ enterprise/
├─ examples/
│  ├─ quote-generation/
│  ├─ inventory-risk/
│  └─ document-analysis/
└─ docs/
```

## Directory Roles

### kernel

Contains OS-level decision modules.

- `goal-kernel`: owns goals, tasks, plans, and scheduling.
- `policy-kernel`: owns permission, approval, and risk decisions.
- `context-core`: owns system context and memory.

### runtime

Contains controlled execution environments.

- `agent-runtime`: lifecycle and isolation for agent processes.
- `execution-runtime`: workflow execution, retry, rollback, verification.
- `model-runtime`: local and cloud model selection.
- `tool-runtime`: loading and invoking tools through Tool ABI.

### shells

Contains device-specific user surfaces.

The shell adapts to screen size, input mode, battery, network, and usage
context. Shells do not own business logic.

### specs

Contains stable protocols and schemas.

This is the most important area early in the project. AIOS should first define
its objects and protocols before building a large UI.

### services

Contains system services used by the kernel and runtime.

### plugins

Contains capability adapters.

A plugin is not a UI feature. It is a controlled system capability exposed
through the Tool ABI.

### examples

Contains reference flows that prove the system loop.

