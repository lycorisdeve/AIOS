# AIOS Architecture

## Positioning

AIOS is an operating system for goal-driven computing.

It is not a chatbot, not a desktop launcher, and not only an agent platform.
The defining idea is that agents become system-level processes, and goals become
the primary way users interact with the system.

## System View

```text
┌─────────────────────────────────────────────────────────┐
│                         User                            │
│        text / voice / vision / gesture / device input    │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                      AIOS Shell                         │
│  adaptive UI, task surface, evidence view, approvals     │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                     Goal Kernel                         │
│   goal parsing, task graph, planning, agent scheduling   │
└──────────────┬────────────────┬────────────────┬────────┘
               │                │                │
               ▼                ▼                ▼
┌────────────────────┐ ┌────────────────┐ ┌────────────────┐
│   Agent Runtime    │ │ Policy Kernel  │ │  Context Core  │
│ lifecycle/isolation│ │ permission/risk│ │ system context │
└──────────┬─────────┘ └────────┬───────┘ └────────┬───────┘
           │                    │                  │
           ▼                    ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│                  Execution Runtime                      │
│ workflow, tool routing, sandbox, retry, rollback, verify │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                       Tool ABI                          │
│ local files, browser, email, ERP, CRM, DB, devices, APIs │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              Resource and Model Scheduler               │
│ CPU, GPU, NPU, memory, network, model, token, API quota  │
└───────────────────────────┬─────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│          Hardware / Existing OS / Cloud / Enterprise     │
└─────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### AIOS Shell

The shell is the user-facing surface. It is adaptive, not fixed.

It provides:

- goal input
- voice and text interaction
- task monitoring
- evidence inspection
- artifact preview
- approval and confirmation
- cross-device continuation

The shell is not the OS. It is only the visible surface of AIOS.

### Goal Kernel

The Goal Kernel owns the system-level execution graph.

It turns:

```text
Goal -> Task -> Plan -> AgentProcess -> ToolCall -> Evidence -> Artifact
```

It is responsible for:

- goal normalization
- task decomposition
- task graph generation
- agent selection and scheduling
- state transitions
- dependency management
- failure classification

### Agent Runtime

The Agent Runtime treats agents like OS processes.

It manages:

- start
- pause
- resume
- cancel
- terminate
- context isolation
- capability assignment
- resource limits
- behavior logging

An agent is not allowed to freely act. It must operate inside a scoped runtime.

### Policy Kernel

The Policy Kernel decides what is allowed.

It evaluates:

- user identity
- organization role
- active goal
- current task
- tool capability
- data sensitivity
- risk level
- approval requirement
- rollback availability

Policy is checked before tool calls, before artifact release, and before any
high-impact action.

### Context Core

The Context Core stores system-level context.

It includes:

- user context
- device context
- enterprise context
- task context
- historical context
- permission context
- data lineage

This is not a chat memory. It is an OS context substrate.

### Execution Runtime

The Execution Runtime turns planned work into controlled actions.

It provides:

- workflow execution
- tool routing
- tool sandboxing
- retry
- rollback
- simulation
- verification
- compensation
- audit emission

### Tool ABI

The Tool ABI is the equivalent of system calls for AIOS.

Every tool declares:

- name
- input schema
- output schema
- permission scope
- risk level
- side effects
- rollback behavior
- approval requirement
- audit requirement

### Resource and Model Scheduler

The scheduler manages traditional and AI-native resources:

- CPU
- GPU
- NPU
- memory
- storage
- network
- local models
- cloud models
- API quota
- token budget
- latency budget
- battery budget

## Core Principle

```text
Agent is the process.
Tool is the syscall.
Goal is the entrypoint.
Policy is the boundary.
Evidence is the proof.
Artifact is the result.
Audit is the memory.
```

