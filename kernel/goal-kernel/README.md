# Goal Kernel

The Goal Kernel owns AIOS goal execution.

Responsibilities:

- normalize user goals
- decompose goals into task graphs
- generate plans
- schedule agent processes
- track state transitions
- emit audit events

The Goal Kernel should not directly call external systems. It schedules work
through the Execution Runtime and Tool ABI.

