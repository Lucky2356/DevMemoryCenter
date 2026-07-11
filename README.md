# Dev Recall

[![CI](https://github.com/Lucky2356/DevMemoryCenter/actions/workflows/ci.yml/badge.svg)](https://github.com/Lucky2356/DevMemoryCenter/actions/workflows/ci.yml)

Local memory for developers and system administrators.

Dev Recall is a planned local-first desktop application for restoring context around local development work. The repository currently contains the Phase 0 architecture and a minimal application foundation only; MVP features are not implemented yet.

## Current state

- Windows and Linux are the target platforms.
- The selected stack is Rust, Tauri 2, React, strict TypeScript, Vite, and SQLite.
- Data collection, shell integration, command execution, networking, telemetry, and automatic updates are absent from the initial skeleton.
- Product requirements are defined in [SPEC.md](SPEC.md); the executable plan is [TODO.md](TODO.md).

## Development

See [docs/development/BUILDING.md](docs/development/BUILDING.md), [docs/development/LOCALIZATION.md](docs/development/LOCALIZATION.md), and [docs/development/UI_FOUNDATION.md](docs/development/UI_FOUNDATION.md). Security and privacy constraints are described in [SECURITY.md](SECURITY.md), [THREAT_MODEL.md](THREAT_MODEL.md), and [PRIVACY.md](PRIVACY.md).

## License

All rights reserved — license decision pending. See [LICENSE](LICENSE) and [NEEDS_USER_INPUT.md](NEEDS_USER_INPUT.md).
