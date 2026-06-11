<p align="center">
  <img src="docs/assets/aios-logo.svg" alt="AIOS logo" width="120" />
</p>

<h1 align="center">AIOS</h1>

<p align="center">
  <strong>AI 不是一个应用。AI 是操作系统界面。</strong>
</p>

<p align="center">
  <a href="README.md">English</a>
  ·
  <a href="README.zh-CN.md">简体中文</a>
  ·
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <img src="docs/assets/aios-homepage-preview.svg" alt="AIOS homepage concept preview" width="100%" />
</p>

## AIOS 是什么？

AIOS 是一个实验性的 AI 原生操作系统架构项目。

它不是聊天助手，不是网页前端，也不是 Electron 套壳。AIOS 想探索一种新的系统模型：

```text
用户意图
-> AIOS Core
-> Policy Kernel
-> 原生应用调用
-> 用户确认
-> 可审计执行
```

用户不应该从“打开应用”开始。用户应该表达意图，系统负责调用对应的原生应用、工具、权限策略、预览界面和确认流程。

例如：

```text
告诉张三，今晚吃瓦香鸡
```

传统系统：

```text
打开聊天应用 -> 找到张三 -> 输入消息 -> 检查 -> 发送
```

AIOS：

```text
理解意图
-> 调用聊天应用
-> 展示消息预览
-> 等待确认
-> 发送
-> 记录审计
```

## 核心理念

- AI 是系统界面，不是另一个 App。
- 应用仍然存在，但由意图调用。
- Agent 是受管理的系统进程。
- Tool 是受控系统能力。
- Policy 定义 AI 行为边界。
- Evidence 和 Audit 是系统可信度的一部分。

## 技术方向

主线：

```text
Rust       -> AIOS Core、Kernel、Runtime、Native Shell Model
WASM       -> 插件沙箱和可移植工具能力
C / C++    -> 必要的底层平台和驱动互操作
Python     -> 模型实验和评测，不进入可信核心
```

非主线：

```text
Node       -> 不作为核心依赖
Electron   -> 不作为目标 Shell
HTML UI    -> 仅作为视觉参考
Web 后端   -> 不是核心架构
```

## 当前状态

仓库目前包含：

- Rust workspace 骨架
- 核心系统对象类型
- Goal Kernel 原型
- Policy Kernel 原型
- Agent Runtime 原型
- Native Shell 状态模型
- Native Platform 抽象
- Tool ABI 和 workflow schemas
- 首页概念图
- 架构、路线图和设计文档

## 本地运行

安装 Rust 后运行：

```bash
cargo test --workspace
```

运行 CLI 原型：

```bash
cargo run -p aios-cli -- "告诉张三，今晚吃瓦香鸡"
```

运行 daemon 原型：

```bash
cargo run -p aios-daemon
```

## 共创方向

欢迎从这些方向参与：

- Kernel 和 Runtime 设计
- Native Shell 模型
- 平台适配
- 权限和安全模型
- Tool ABI 与插件沙箱
- 真实意图场景
- 架构文档
- 视觉系统设计

AIOS 还很早期，但方向很清晰：

```text
下一代操作系统，不应该只是多一个 AI 助手。
它应该让 AI 成为系统级界面，让应用成为可被意图调用的能力。
```

欢迎提 Issue 或 PR，一起共创 AIOS。

## 关键文档

- [Architecture](docs/ARCHITECTURE.md)
- [Native OS Strategy](docs/NATIVE_OS_STRATEGY.md)
- [Technology Stack](docs/TECH_STACK.md)
- [Core Objects](docs/CORE_OBJECTS.md)
- [Adaptive Shell](docs/ADAPTIVE_SHELL.md)
- [Homepage Effect](docs/HOMEPAGE_EFFECT.md)
- [MVP Roadmap](docs/MVP_ROADMAP.md)

## License

MIT License.

