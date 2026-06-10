# Homepage Effect

The first screen of AIOS should communicate one thing immediately:

```text
AI is the operating-system interface.
Applications are invoked around it.
```

## First View

```text
┌──────────────────────────────────────────────────────────────┐
│ AIOS                                      Wi-Fi  82%  09:41   │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  System Context       AIOS Core               Invoked App     │
│  ┌────────────┐   ┌────────────────┐    ┌────────────────┐   │
│  │Execution   │   │告诉张三，今晚...│    │ 微信           │   │
│  │Policy      │   │                │    │ 消息预览       │   │
│  │Audit       │   │ floating input │    │ 确认发送       │   │
│  └────────────┘   └────────────────┘    └────────────────┘   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Visual Rules

- AIOS Core is the center of the homepage.
- Apps are not opened manually by the user.
- Apps appear as invoked native surfaces.
- The user sees the result and confirms the action.
- Policy, execution, and audit are visible but secondary.

## Example

User intent:

```text
告诉张三，今晚吃瓦香鸡
```

Homepage response:

```text
AIOS Core
-> understands message intent
-> invokes chat app
-> shows WeChat-like message preview
-> waits for user confirmation
```

The homepage should not look like a normal website, app launcher, or chatbot.
It should look like a system-level intent surface.

