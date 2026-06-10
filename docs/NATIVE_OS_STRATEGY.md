# Native OS Strategy

AIOS should not be treated as a web frontend/backend project.

The HTML prototype exists only to communicate visual intent. It is not the final
shell technology.

## Final Direction

```text
AIOS Core:
  Rust

Native shell model:
  Rust

Platform rendering backends:
  Windows: Win32 / WinUI / DirectComposition / Direct2D
  macOS: AppKit / SwiftUI / CoreAnimation / Metal bridge
  Linux: Wayland compositor protocol / GTK / Qt / wgpu path
  Android: Kotlin / Jetpack Compose bridge to AIOS Core
  iOS: SwiftUI / UIKit bridge to AIOS Core

Plugin sandbox:
  WebAssembly

Model experiments:
  Python outside the trusted core
```

## What Node Is For

Node should not be required by the AIOS core.

It may be used only outside the product path for optional design tooling, if
needed. The repository should not depend on Node to build or validate the native
core.

## Shell Principle

AI is the interface center.

Applications are invoked by intent:

```text
User intent
-> AIOS Core
-> Policy Kernel
-> Native App Invocation
-> Native App Preview
-> User Confirmation
-> Execute
```

Example:

```text
Tell Zhang San: dinner is waxiang chicken
```

AIOS should:

1. understand the user intent
2. select the chat application
3. prepare a native message preview
4. request confirmation
5. send the message through the native app/tool boundary
6. audit the action

The user should not manually open an app first.

