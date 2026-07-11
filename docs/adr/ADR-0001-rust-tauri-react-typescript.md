# ADR-0001: Rust, Tauri, React, and TypeScript Stack

## Status

Accepted

## Context

Dev Recall needs a lightweight Windows/Linux desktop UI with a small privileged surface, strong backend safety, and accessible web presentation.

## Decision

Use stable Safe Rust (edition 2024), Tauri 2, React, strict TypeScript, Vite, and CSS Modules. Keep Tauri as the composition root rather than a domain framework.

## Considered alternatives

- Electron: broader runtime and attack/resource surface; rejected by requirements.
- Native platform UIs: duplicate Windows/Linux presentation work.
- A heavy component library or Tailwind: unnecessary for the initial shell; CSS Modules has less dependency/configuration cost.

## Security implications

Rust reduces memory-safety risk; Tauri capabilities and IPC still require strict design. Remote content, shell plugin, dynamic code, and unsafe HTML remain forbidden.

## Privacy implications

The stack works fully locally and requires no telemetry or hosted assets.

## Performance implications

Tauri avoids bundling a full browser runtime but packaged size/startup must be measured on both platforms.

## Consequences

Developers need Rust, Node, platform native build tools, and WebView prerequisites. Frontend/backend contracts require explicit types and runtime validation.
