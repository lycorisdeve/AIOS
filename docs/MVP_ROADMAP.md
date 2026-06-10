# MVP Roadmap

The first version should prove AIOS as an operating system layer, not as a
general assistant.

## MVP Goal

Prove this loop:

```text
Goal
-> Task Graph
-> Agent Process
-> Policy Check
-> Tool Call
-> Verification
-> Artifact
-> Audit
```

## MVP Scope

Pick one business-grade scenario.

Recommended first scenario:

```text
Inventory risk analysis
```

Why:

- read-heavy
- clear business value
- low destructive risk
- easy to verify
- evidence can be shown clearly
- can later connect to ERP/MES/CRM

## MVP Modules

### Phase 1: Protocol First

Deliver:

- core object schemas
- Tool ABI schema
- policy decision schema
- workflow state schema
- audit record schema

### Phase 2: Local Runtime

Deliver:

- basic Goal Kernel
- basic Agent Runtime
- basic Execution Runtime
- mock Tool ABI
- local audit log

### Phase 3: Adaptive Shell

Deliver:

- desktop shell
- mobile shell prototype
- shared task status components
- evidence viewer
- artifact preview

### Phase 4: Real Tool Integration

Deliver:

- file plugin
- database plugin
- HTTP API plugin
- enterprise API example

### Phase 5: Verification and Approval

Deliver:

- risk classification
- pre-execution policy check
- post-execution verification
- approval queue
- rollback/compensation pattern

## Non-Goals

The MVP should not attempt:

- replacing Linux or Windows kernels
- managing all hardware drivers
- supporting every desktop application
- building a full app store
- automating high-risk write operations without approval

## Success Criteria

The MVP succeeds when AIOS can:

- accept a goal
- decompose it into tasks
- start agent processes
- call tools through a declared ABI
- enforce policy before actions
- produce evidence-backed artifacts
- keep an audit trail
- resume the same goal on another shell

