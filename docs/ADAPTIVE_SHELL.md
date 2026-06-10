# Adaptive Shell

AIOS should use one system core across devices.

It should not split into separate systems:

```text
Phone AIOS
Desktop AIOS
Tablet AIOS
Car AIOS
```

It should be:

```text
One AIOS Core
+ Adaptive Device Shells
```

## Principle

Shared:

- Goal Kernel
- Agent Runtime
- Policy Kernel
- Context Core
- Tool ABI
- Execution Runtime
- Semantic File System
- Audit and Evidence

Adaptive:

- layout
- input mode
- density
- notification style
- local model usage
- approval surface
- artifact preview

## Device Context

The shell adapts based on:

- screen size
- pointer availability
- keyboard availability
- voice availability
- camera availability
- network quality
- battery state
- mobility state
- privacy setting
- local compute capability

## Desktop Shell

Desktop is optimized for dense work.

Surfaces:

- task graph
- evidence panel
- artifact editor
- approval queue
- multi-window workspace
- keyboard-first commands

## Mobile Shell

Mobile is optimized for quick decisions and capture.

Surfaces:

- voice goal input
- short task status
- notification approval
- camera-based capture
- artifact summary
- handoff to desktop

## Tablet Shell

Tablet is optimized for review and mixed input.

Surfaces:

- touch-first task board
- document annotation
- visual evidence review
- pen input

## Car Shell

Car shell is optimized for voice and safety.

Surfaces:

- voice-only goal input
- read-aloud summaries
- safe confirmations
- no complex editing while moving

## Continuity

The same goal should continue across devices.

Example:

```text
Phone:
  "Analyze this supplier quote"
  -> capture document with camera

Desktop:
  continue same Goal ID
  -> inspect evidence and approve final report
```

The shell changes. The goal does not.

