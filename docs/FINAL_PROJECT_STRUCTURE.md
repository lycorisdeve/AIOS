# Final Project Structure

This is the target engineering structure for a full AIOS implementation.

```text
AIOS
├─ Cargo.toml
├─ README.md
├─ docs/
│  ├─ ARCHITECTURE.md
│  ├─ ADAPTIVE_SHELL.md
│  ├─ CORE_OBJECTS.md
│  ├─ FINAL_PROJECT_STRUCTURE.md
│  ├─ MVP_ROADMAP.md
│  ├─ PROJECT_STRUCTURE.md
│  ├─ TECH_STACK.md
│  ├─ NATIVE_OS_STRATEGY.md
│  └─ UI_REFERENCE.md
├─ crates/
│  ├─ aios-goal-kernel/
│  ├─ aios-policy-kernel/
│  ├─ aios-context-core/
│  ├─ aios-agent-runtime/
│  ├─ aios-execution-runtime/
│  ├─ aios-tool-runtime/
│  ├─ aios-model-runtime/
│  ├─ aios-resource-scheduler/
│  ├─ aios-platform/
│  ├─ aios-native-shell/
│  ├─ aios-artifact-service/
│  ├─ aios-audit-service/
│  ├─ aios-identity-service/
│  ├─ aios-sync-service/
│  ├─ aios-daemon/
│  └─ aios-cli/
├─ shells/
│  ├─ desktop-shell/
│  │  ├─ src/
│  │  ├─ src-tauri/
│  │  └─ package.json
│  ├─ mobile-shell/
│  │  ├─ src/
│  │  └─ package.json
│  ├─ tablet-shell/
│  ├─ car-shell/
│  └─ shared-shell-kit/
├─ packages/
│  ├─ sdk-rust/
│  ├─ protocol-types/
│  └─ plugin-devkit/
├─ specs/
│  ├─ objects/
│  ├─ policy/
│  ├─ tool-abi/
│  ├─ workflow/
│  └─ protobuf/
├─ plugins/
│  ├─ file-system/
│  ├─ browser/
│  ├─ email/
│  ├─ database/
│  └─ enterprise/
├─ services/
│  ├─ artifact-service/
│  ├─ audit-service/
│  ├─ identity-service/
│  └─ sync-service/
├─ model-lab/
│  ├─ adapters/
│  ├─ evaluations/
│  ├─ prompts/
│  └─ datasets/
├─ prototypes/
│  └─ shell-ui/        visual reference only, not product runtime
├─ examples/
│  ├─ document-analysis/
│  ├─ inventory-risk/
│  └─ quote-generation/
└─ tests/
   ├─ contract/
   ├─ policy/
   ├─ runtime/
   └─ shell/
```

## Why This Structure

### crates

The Rust workspace contains the trusted system core.

This is where AIOS becomes OS-like:

- scheduling
- permission
- runtime isolation
- audit
- resource control
- tool execution

### shells

Device shells are not separate operating systems.

They share AIOS Core and adapt the interaction model:

- desktop: dense work
- mobile: quick capture and approval
- tablet: review and annotation
- car: voice-first safe interaction

### packages

Packages provide SDKs and shared developer tools.

They make Tool ABI and system objects usable from apps, plugins, and shells.

### specs

Specs are the stable contract layer.

This layer should be treated like system ABI. Changes here must be deliberate.

### model-lab

Model work is outside the trusted core.

This prevents experiments from polluting the OS safety boundary.

### prototypes

Fast UI and interaction experiments live here.

Prototype code should inform the production shell, but not define the kernel.
