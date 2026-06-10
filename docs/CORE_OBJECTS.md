# Core Objects

AIOS uses AI-native system objects.

## Object Map

```text
Goal
 └─ Task
     └─ Plan
         └─ AgentProcess
             └─ ToolCall
                 ├─ Evidence
                 └─ Artifact
```

Supporting objects:

```text
Policy
Context
State
AuditRecord
Capability
ResourceBudget
Approval
VerificationResult
```

## Goal

A user-level desired outcome.

```json
{
  "id": "goal_001",
  "title": "Analyze inventory risk",
  "intent": "Find products likely to run out in the next 30 days",
  "owner": "user_001",
  "priority": "normal",
  "state": "active"
}
```

## Task

A decomposed unit of work under a goal.

```json
{
  "id": "task_001",
  "goal_id": "goal_001",
  "title": "Query current inventory",
  "depends_on": [],
  "state": "pending"
}
```

## Plan

An executable strategy.

```json
{
  "id": "plan_001",
  "goal_id": "goal_001",
  "steps": [
    "query_inventory",
    "query_sales_forecast",
    "calculate_risk",
    "generate_report"
  ]
}
```

## AgentProcess

A managed execution process.

```json
{
  "id": "agent_001",
  "task_id": "task_001",
  "agent_type": "analysis_agent",
  "capabilities": ["inventory.read", "forecast.read"],
  "resource_budget": {
    "max_runtime_ms": 120000,
    "max_tokens": 20000
  },
  "state": "running"
}
```

## ToolCall

A controlled system capability invocation.

```json
{
  "id": "toolcall_001",
  "agent_process_id": "agent_001",
  "tool": "erp.inventory.query",
  "input": {
    "warehouse": "all"
  },
  "risk_level": "low",
  "state": "completed"
}
```

## Evidence

Proof used to support a result.

```json
{
  "id": "evidence_001",
  "source": "erp.inventory",
  "tool_call_id": "toolcall_001",
  "summary": "Product A has 500 units available",
  "confidence": 0.96
}
```

## Artifact

A produced result.

```json
{
  "id": "artifact_001",
  "type": "report",
  "title": "Inventory Risk Report",
  "goal_id": "goal_001",
  "evidence_ids": ["evidence_001"],
  "state": "ready_for_review"
}
```

## Policy

A rule that controls action.

```json
{
  "id": "policy_001",
  "scope": "erp.purchase.write",
  "condition": "amount > 10000",
  "decision": "requires_approval"
}
```

## AuditRecord

An immutable behavior record.

```json
{
  "id": "audit_001",
  "actor": "agent_001",
  "action": "tool_call",
  "target": "erp.inventory.query",
  "result": "allowed",
  "timestamp": "2026-06-10T00:00:00Z"
}
```

