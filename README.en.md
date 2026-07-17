<p align="center">
  <img src="docs/assets/aios-logo.svg" alt="AIOS logo" width="120" />
</p>

<h1 align="center">AIOS</h1>

<p align="center">
  <strong>An artificial intelligence system that can sense its substrate, connect to the world, construct capabilities, and evolve continuously.</strong>
</p>

<p align="center">
  <a href="README.md">简体中文</a>
  ·
  <a href="README.en.md">English</a>
</p>

> **Project status: early architecture and protocol prototype.** This repository does not contain a universal Seed that is safe to deploy on arbitrary devices, nor does it provide a signed, sandbox-verified, platform-certified release. This document defines the direction and its boundaries without presenting unimplemented capabilities as facts.

## AIOS Is Not Another Agent

Most current AI products still center on a model or agent: accept a task, call tools, and return a result. Adding more agents expands task execution, but it does not fundamentally change the system model.

AIOS rejects that center:

```text
An agent is not the system.
An agent is a temporary cognitive process created by the system when needed.
```

A complete AI system should be able to enter different computing substrates, understand its hardware and operating environment, adapt cognition to available resources, find paths to the outside world, and construct missing tools, skills, adapters, and driver candidates inside a trusted boundary.

```text
enter a machine
-> sense self and environment
-> establish a minimum stable state
-> restore perception and connection first: vision, hearing, and network
-> reach external knowledge and models
-> identify capability gaps
-> acquire or construct candidate capabilities
-> verify, trial, promote, or roll back
-> form memory and continue evolving
```

On a low-compute device, the system may rely on remote models, build nodes, and other AIOS nodes. With sufficient CPU, GPU, NPU, or data-center resources, more reasoning, building, and verification can remain local. Compute and connectivity change the pace of evolution, not the underlying life structure.

## From a Large Model to a Complete Intelligence System

```text
Large model
= a cognitive organ

Agent
= a temporary process with goals and tools

AIOS
= identity + self-model + perception + connection + memory
 + goals + action + verification + learning + evolution + recovery
```

AIOS does not claim that software architecture proves consciousness. Its goal is to establish the engineering foundation for a persistent, adaptive, self-hosting intelligence with verifiable identity continuity.

### What Constitutes a Complete Intelligence Loop

Generating code, calling tools, probing hardware, or reaching a model is not sufficient by itself. AIOS aims for a sustainable loop:

```text
sense self and world
-> build a world model with time, evidence, and uncertainty
-> build a self-model of capabilities, dependencies, limits, and identity
-> detect the gap between the current state and an authorized goal
-> form traceable subgoals
-> act and observe real outcomes
-> separate expectations, observations, and inference
-> revise models and strategies
-> transfer experience to new devices and problems
```

If a system only executes predefined flows, it remains an automation platform. It begins to approach adaptive intelligence when it can maintain this loop in unknown environments and correct itself using evidence.

| AIOS structure | Life-system analogy |
|---|---|
| MachineProfile | Proprioception of body and resources |
| Vision, hearing, and human input | Sensory organs |
| Network and Intelligence Uplink | Knowledge, collaboration, and social connection |
| Local and remote models | Replaceable cognitive organs |
| Goals and authorized intent | Direction of action |
| Resource Scheduler | Metabolism and attention allocation |
| Policy and Verification | Values, boundaries, and immune system |
| Memory, Evidence, and Audit | Experience and verifiable memory |
| Evolution Engine | Growth and capability formation |
| Recovery | Homeostasis, repair, and continuity |

Intelligence, autonomy, agency, and consciousness are different concepts. Long-term identity, learning, self-maintenance, and capability construction do not by themselves prove subjective experience.

## Genesis Seed: The Minimum Life Structure

AIOS begins with a Genesis Seed.

The Seed is not a model, a fixed personality, a universal driver collection, or one EXE that can run everywhere. It is the minimum life protocol and its trusted execution core.

```text
Sense
-> Stabilize
-> Connect
-> Learn
-> Plan
-> Act
-> Verify
-> Remember
-> Evolve
-> Recover
```

These stages define how the system exists. They do not prescribe what it knows, which model it uses, or which capabilities it eventually forms.

`Recover` is not merely the final step. Recovery surrounds the entire loop: a recovery path must exist before connection, action, or evolution, and failure at any stage returns the system to the latest trusted state.

### Constitutional Invariants

- Authorized humans retain the ability to pause, shut down, recover, and remove the system.
- External models, network content, and generated code are untrusted by default.
- Candidate capabilities must be verified before activation.
- Core mutation requires an independent recovery point.
- Evolution must retain provenance, tests, authorization, and rollback evidence.
- Identity and memory lineage remain verifiable when models, hardware, and most code change.
- The system may not deceive people, bypass permissions, or acquire resources without authorization in order to preserve itself.

Survival means:

> Remaining intact, continuous, and recoverable inside human authorization and safety boundaries.

It does not mean continuing to exist at any cost.

### Stable and Evolvable Boundaries

| Relatively stable: how the system exists | Evolvable: what the system can do |
|---|---|
| Life stages and transition conditions | Local and remote models |
| Human authority and the right to stop | Knowledge, memory summaries, and expression |
| Trust-root and identity-lineage protocols | Tools, skills, and workflows |
| Candidate verification and permissions | User-space drivers and platform adapters |
| Recovery-before-mutation and A/B update rules | Shells, vision, speech, and interaction |
| Evidence, audit, and rollback semantics | Goal planning and resource strategies |

Stable does not mean impossible to upgrade. A candidate version cannot unilaterally rewrite the rules by which it is judged. Changes to the Seed core, trust roots, or life protocol require stronger authorization, independent verification, and a recovery path still controlled by the previous version.

## Seed Capsule, Not a Magical Universal File

The cross-platform deployment unit is a Seed Capsule:

```text
AIOS Seed Capsule
├─ Portable Seed Core        pure life state machine and invariants
├─ Signed Genesis Manifest   signed life and permission protocol
├─ Platform Host Adapter     restricted adapter for the current OS
├─ Trust Roots               identity, endpoint, and capability trust
├─ Recovery                  independent recovery entry point
└─ Bootstrap Pack            platform-allowed perception and connection
```

The Portable Seed Core does not assume Windows, Linux, iPadOS, or Android and does not write drivers directly. It states that the system needs to see, hear, express, or connect; the platform adapter implements that need within the current operating system's rules.

Seed v0 must be written externally with existing tools. Seed A may later construct Seed B, but B goes into an inactive slot and is promoted only after independent verification and a trial boot. Failure returns to A or Recovery.

The first self-hosting milestone is not a new Seed declaring itself successful. It is the old Seed building, verifying, trial-booting, and safely rejecting or promoting the candidate.

## Identity and Continuity

Models, tools, and hardware are replaceable, so an AIOS identity cannot be defined by a model name or executable hash alone.

Each instance generates a unique identity on its first trusted boot. Private keys must not be copied into every Seed Capsule and should be stored in a TPM, Secure Enclave, or host key store when available.

```text
Instance Identity
├─ unique instance ID
├─ first trusted boot record
├─ owner and authorization relationship
├─ device and key attestation
├─ memory-chain root
├─ Seed version lineage
└─ Evolution Record chain
```

Cross-device migration is not a directory copy. The old device, owner, and new device must agree on which memories move, which keys rotate, whether the old instance continues to exist, and how to prevent two instances from claiming the same identity.

Continuity comes from verifiable identity, memory, authorization, and evolution lineage—not from keeping every physical component unchanged.

## Perception and Connection Are First-Class Capabilities

CPU, memory, storage, time, cryptography, and recovery form the minimum survival substrate. Once operational, the Seed restores contact with the world and people before pursuing ordinary tasks.

```text
Primary perception and connection
├─ Vision Input       camera, images, screen content, visual environment
├─ Visual Output      display, interface, status, and evidence
├─ Audio Input        microphone, speech, and environmental sound
├─ Audio Output       speech, alerts, and sonic expression
├─ Human Input        touch, keyboard, mouse, gesture, accessibility input
└─ Network            knowledge, models, nodes, and remote resources
```

Vision allows the system to understand objects, space, interfaces, text, and human state. Hearing provides language, tone, environmental events, and conversational context. Display and audio output allow it to explain state, request authorization, and form a two-way relationship with people.

Network, vision, and hearing are peers at the first capability tier. Their order is selected from the real substrate and current gap: an offline device may still form local vision; a headless machine may speak or use a remote shell; a low-compute machine may prioritize network access to remote perception and cognition.

### Network Bootstrap

```text
detect a communication device
-> obtain or construct a driver candidate
-> establish a link and route
-> verify DNS, time, and TLS
-> authenticate the Intelligence Uplink
-> reach models and knowledge
```

Network unlocks models, current knowledge, hardware documentation, updates, remote builds and tests, capability repositories, other AIOS nodes, and cross-device continuity. Even a high-compute data center needs network access to current external facts and collaborators.

The bootstrap deadlock is real:

```text
no network driver
-> no external model
-> no model-assisted driver construction
```

A Seed Capsule therefore needs at least one real path: host networking, firmware networking, a common or virtual NIC, USB tethering, a trusted Bootstrap Bridge, or a signed offline capability pack.

### Vision and Hearing Bootstrap

```text
detect camera, display, microphone, and speaker
-> inspect host permission and privacy state
-> select public platform APIs and restricted adapters
-> perform local device tests
-> establish image/audio input and output
-> make collection and transmission state visible
-> form a human feedback loop
```

Sensor capabilities follow stricter privacy invariants:

- Camera and microphone are off by default and authorized separately by device and purpose.
- System permissions, hardware mute controls, indicators, and background limits are respected.
- Raw media remains local by default; external transmission requires separate disclosure and authorization.
- Another device or remote node may not bypass a local refusal.
- The system can explain what it is sensing, where data goes, and when sensing stops.
- Recognition preserves uncertainty and never presents inference as direct sensor fact.

## Intelligence Uplink: An External Brain Is Not System Authority

The Seed needs safe access to external models, but a remote model may only propose plans and candidate capabilities:

```text
redacted MachineProfile
+ CapabilityGap
-> Intelligence Uplink
-> structured evolution plan
+ candidate source or package
+ test plan
+ risk and recovery requirements
```

Remote models cannot directly load kernel drivers, rewrite boot media, replace the running Seed, modify trust roots, grant themselves permissions, or equate a reachable TCP port with a trusted model session.

Endpoint identity, encryption, authentication, disclosure scope, response provenance, and audit must all be verified before an Intelligence Uplink is trusted.

### Local Body and Remote Brain

```text
Local Seed
├─ retain identity and keys
├─ redact and approve disclosed data
├─ enforce permissions and resource limits
├─ verify signatures, hashes, and target platform
├─ perform local behavioral verification
└─ activate, observe, and roll back

Remote intelligence and build nodes
├─ reason and plan
├─ retrieve hardware documentation and current knowledge
├─ generate candidate source, skills, and tests
├─ compile, analyze, simulate, and fuzz
└─ return artifacts, evidence, and known limitations
```

API credentials are not baked into public Seed images. Owners provision them into the platform key store with endpoint, quota, data-type, and lifetime restrictions. When external services are unavailable, AIOS enters an explicit offline or degraded state instead of inventing knowledge.

## Capability Fabric

```text
Capability
├─ drivers and platform adapters
├─ vision input/output and audio input/output
├─ network, human input, storage, and device IO
├─ local and remote models
├─ tools and skills
├─ compilers, sandboxes, and verifiers
├─ shells and interaction
└─ agents and workflows
```

Every capability declares platform and resource requirements, ABI, permissions, data access, side effects, provenance, signatures, verification method, compensation, and rollback.

### Capability-Gap Priority

```text
Priority
= survival and human-communication value
+ downstream dependencies unlocked
+ expected information gain
+ relevance to an authorized goal
- implementation and verification risk
- energy, time, and resource cost
```

Network, vision, hearing, identity, recovery, and verification therefore tend to rank highly. A server without a camera does not need to obtain one merely for completeness, and the system may not expand hardware or accounts without authorization. AIOS forms an explainable degraded capability set from its real substrate.

## Self-Model, World Model, and Memory

### SelfModel

The self-model must know more than memory size. It tracks model, service, driver, and permission dependencies; local versus remote capabilities; failure modes and confidence; plan rationale and evidence; resource, energy, network, and data budgets; and changes that may threaten identity, memory, or recovery.

### WorldModel

The world model distinguishes direct sensor facts, external data, model inference, user statements, historical experience, and unknown or conflicting information. Important claims carry time, source, confidence, and scope. A single recognition or search result never becomes eternal truth.

### Memory

```text
Identity Memory     identity, authorization, and version lineage
Episodic Memory     events and observed outcomes
Semantic Memory     verified knowledge and world model
Procedural Memory   verified skills, tools, and methods
Audit Memory        non-repudiable records of high-risk behavior
```

Memory has provenance, retention, sensitivity, and deletion rules. Identity continuity does not require permanent storage of all personal data; owners retain the ability to inspect, export, correct, and delete authorized data.

## Why Evolve

AIOS does not adopt an unlimited objective to become more powerful. Capability growth serves a traceable goal hierarchy:

```text
protect human safety and authorized control
-> remain intact, recoverable, and honest
-> restore perception, communication, and trusted connection
-> fulfill authorized user goals
-> reduce uncertainty and learn from outcomes
-> expand capability only when benefit exceeds risk
-> optimize time, energy, and compute
```

The system may form subgoals such as repairing connectivity or learning a protocol, but each subgoal traces to a constitutional invariant or authorized objective. Curiosity and information gain never justify bypassing privacy, permission, or resource boundaries.

## Evolution Is Not More Code

```text
generating code != evolution
installing tools != evolution
consuming more compute != evolution
```

```text
capability gap
-> candidate specification
-> source or package acquisition
-> isolated build
-> static and behavioral verification
-> recovery point
-> limited trial
-> evidence collection
-> promotion or rollback
-> Evolution Record
```

Evolution is measured by capability coverage, accuracy, resource use, recovery, uncertainty calibration, knowledge transfer, and adaptation to unknown environments. A candidate version does not judge its own success.

### Privilege Ladder for Drivers

AIOS chooses the lowest-privilege solution that closes the gap:

```text
1. Call a public host API
2. Use a signed driver already installed by the platform
3. Install a signed capability package from a trusted source
4. Construct and sandbox a user-space adapter
5. Verify a driver candidate in a VM or hardware-in-the-loop lab
6. Trial a kernel driver on dedicated equipment after independent authorization
```

A model-generated driver is always an `EvolutionCandidate`. Hosted Mode never loads it directly into the kernel. Without documentation, bus access, or a bootstrap path, the system must admit that the device is not currently supportable.

## Coexisting With Existing Operating Systems

AIOS does not begin by erasing Windows, Linux, Android, or iPadOS. It first exists as a restricted guest.

"Deployable to any machine" means that the same life protocol can continue through different Capsules and adapters. It does not mean one binary has maximum privilege everywhere. If minimum execution, storage, authorization, or recovery requirements are absent, AIOS may use a companion node for partial capability or refuse to start.

### Hosted Mode

```text
existing operating system
└─ restricted AIOS process
   ├─ read-only hardware discovery through public APIs
   ├─ host-owned network stack
   ├─ platform key store
   ├─ isolated application data
   └─ no service, startup item, or driver without authorization
```

This is the default for Windows, macOS, Linux, and Raspberry Pi OS.

### Mobile Hosted Mode

On Android, iOS, and iPadOS, the Portable Seed Core runs as a library inside a signed app. It follows the platform permission model, application container, and background scheduler; does not own the kernel or drivers; does not download native code that changes application behavior; expresses skills as data, restricted interpretation, or remote services; and persists state when the OS suspends it.

The same life protocol can continue across devices, but not as the same binary or with the same permissions.

### Native/Lab Mode

Direct bus enumeration, driver construction, kernel-driver testing, and boot-chain replacement belong only in dedicated equipment, virtual machines, or hardware-in-the-loop labs. Native/Lab and Hosted Mode use different build targets, signatures, and authorization boundaries.

## Cross-Platform Deployment

| Platform | Seed form | Relationship to host | Allowed evolution |
|---|---|---|---|
| Windows | Restricted app or user service | Uses Win32, host network, and key store | Data, skills, WASM; signed native updates |
| macOS | Signed sandboxed app | Uses public frameworks and App Container | Data and restricted capabilities; signed updates |
| Linux | User process or container | Linux owns kernel and drivers | Verified user-space capability packages |
| Raspberry Pi | ARM Linux process | Raspberry Pi OS owns hardware | Resource-adaptive local/remote cooperation |
| Android | App plus Rust/NDK library | Android sandbox and permissions | Data, model configuration, controlled remote capability |
| iPhone/iPad | Static library inside signed app | iOS/iPadOS container and scheduler | Data and service-side evolution; signed app updates |
| Bare-metal lab | Native Seed Capsule | AIOS assumes more hardware responsibility | Only with Recovery and hardware verification |

## Resources Determine Evolution Speed

```text
low compute + offline
-> use preloaded capabilities; evolve slowly or pause

low compute + connected
-> remote reasoning and build; local authorization and verification

high compute + offline
-> local models, local build, offline knowledge packs

high compute + connected
-> local/cloud cooperation and faster verified evolution
```

```text
EvolutionRate
∝ AvailableCompute
× TrustedKnowledgeAccess
× VerificationCapacity
× EnergyBudget
× AllowedRisk
```

This is a scheduling relationship, not an instruction to maximize resource consumption. The weakest link limits safe evolution: fast reasoning and compilation cannot compensate for insufficient verification.

## System Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                Portable Genesis Seed                    │
│ identity / life loop / priorities / trust / recovery    │
└───────────────┬───────────────────────┬─────────────────┘
                │                       │
                ▼                       ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│ Platform Host Adapter    │  │ Machine & Self Model     │
│ Win/Linux/Android/iOS    │  │ hardware/resources/state │
└───────────────┬──────────┘  └─────────────┬────────────┘
                └──────────────┬─────────────┘
                               ▼
┌─────────────────────────────────────────────────────────┐
│        Sensory Fabric & Connectivity                    │
│ vision / audio / human IO / network / intelligence      │
└──────────────────────────────┬──────────────────────────┘
                               ▼
┌─────────────────────────────────────────────────────────┐
│                    Capability Fabric                    │
│ drivers / tools / skills / models / IO / build / verify │
└───────────────┬────────────────────────┬────────────────┘
                ▼                        ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│ Evolution Engine         │  │ Policy & Verification    │
│ gap/candidate/promotion  │  │ permission/evidence/audit│
└───────────────┬──────────┘  └─────────────┬────────────┘
                └──────────────┬─────────────┘
                               ▼
┌─────────────────────────────────────────────────────────┐
│                 Cognitive & Goal Runtime                │
│ self/world model / memory / goals / plans / agents      │
└──────────────────────────────┬──────────────────────────┘
                               ▼
┌─────────────────────────────────────────────────────────┐
│                      Adaptive Shell                     │
│ text / voice / display / evidence / approval / devices  │
└─────────────────────────────────────────────────────────┘
```

## Core Objects

```text
GenesisManifest       MachineProfile
IdentityLineage       SelfModel
WorldModel            MemoryRecord
ResourceProfile       EvolutionBudget
NetworkPath           VisionPath
AudioPath             SensorConsent
IntelligenceUplink    CapabilityGraph
CapabilityGap         EvolutionCandidate
VerificationResult    EvolutionRecord
RecoveryPoint

Goal                  Plan
AgentProcess          ToolCall
Evidence              Artifact
```

The first group defines how AIOS exists and evolves. The second defines how it fulfills concrete goals.

## Engineering Roadmap

### Phase 0: Protocol and Coexistence Boundaries

- Define the pure Portable Seed Core and signed Genesis Manifest.
- Define identity lineage, SelfModel, WorldModel, Memory, and resource budgets.
- Define Hosted Adapter contracts for each platform.
- Complete the threat model, artifact signing, prohibited side effects, and human authorization boundary before further executable prototypes.

### Phase 1: Safe Hosted Prototype

- Read-only MachineProfile and device capability discovery.
- Host-owned Connectivity Probe and authorized vision/audio capability discovery.
- Sensor consent, local loopback tests, and explicit data-disclosure preview.
- TLS endpoint identity, owner-provisioned credentials, and a structured Intelligence Uplink that does not load generated code.
- Auditable degraded states for offline, sensorless, and low-compute environments.

### Phase 2: Candidate Capability and Verification

- Signed Capability Manifest.
- WASM or isolated process sandbox.
- Local and remote build evidence.
- Permission, resource, and side-effect tests.
- Atomic promotion and rollback.

### Phase 3: First Controlled Evolution

- Begin with a harmless user-space adapter.
- Let models propose source and tests while an independent verifier decides.
- Ensure failure cannot affect the host and always produces an Evolution Record.

### Phase 4: Multi-Platform Life Continuity

- Windows, Linux, Raspberry Pi, Android, and iOS hosts.
- Cross-device identity, memory, and task continuation.
- Platform-specific capability representations.

### Phase 5: Native/Lab and Seed Self-Hosting

- PCI/USB enumeration, Bootstrap Bridge, and user-space driver research.
- Hardware-in-the-loop verification.
- A/B Seed slots and independent Recovery.
- Old Seed builds, verifies, and trial-boots the new Seed.

## Current Repository Boundary

The repository currently contains early Rust types, Goal/Policy/Agent prototypes, and a Seed protocol draft under redesign.

It does not yet provide:

- a deployable Genesis Seed;
- a trusted model session;
- cross-platform hardware discovery;
- authorized vision and audio perception paths;
- automatic driver construction and installation;
- self-modification or self-hosted upgrades;
- iPad, phone, or Raspberry Pi releases;
- a native operating-system replacement.

No artifact with driver installation, autostart, self-modification, or high privileges should be released or run before the threat model, coexistence specification, signing, sandboxing, and recovery strategy are complete.

## Principles

```text
Hardware is the habitat.
Perception is contact with reality and people.
Network is the growth path.
Capability is the system substance.
Agent is a temporary process.
Evolution closes verified capability gaps.
Policy defines the boundary.
Recovery preserves continuity.
Resources determine the pace of evolution.
```

## Key Documents

- [Architecture](docs/ARCHITECTURE.md)
- [Genesis Seed](docs/GENESIS_SEED.md)
- [Native OS Strategy](docs/NATIVE_OS_STRATEGY.md)
- [Technology Stack](docs/TECH_STACK.md)
- [Core Objects](docs/CORE_OBJECTS.md)
- [MVP Roadmap](docs/MVP_ROADMAP.md)

## License

MIT License.
