# UI Reference

The visual direction follows the provided reference image:

```text
soft blue ambient background
glass display panel
minimal floating input bar
voice button
send button
low-noise system surface
```

## Product Feeling

The AIOS shell should feel like:

- calm
- native
- spatial
- light
- continuous across devices
- less like a chat app
- more like a system surface

## Layout

```text
┌─────────────────────────────────────────────┐
│                                             │
│               Glass Display                 │
│      task, artifact, evidence, approval      │
│                                             │
│                                             │
│                                             │
│              Floating Goal Bar              │
└─────────────────────────────────────────────┘
```

## Core UI Objects

### Display Panel

Shows the active system surface:

- active goal
- current artifact
- task progress
- evidence
- approval request
- handoff prompt

### Floating Goal Bar

Primary input surface.

Supports:

- voice
- text
- send
- device-aware suggestions

### Adaptive Surfaces

Desktop:

- larger display panel
- multi-column task/evidence/artifact mode

Mobile:

- compact result card
- bottom input bar
- notification approval

Tablet:

- review canvas
- annotation support

Car:

- voice-first
- simplified confirmations

## Prototype

A static prototype is available at:

```text
prototypes/shell-ui/index.html
```

Open it directly in a browser to preview the visual direction.

