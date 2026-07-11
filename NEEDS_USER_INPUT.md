# Needs User Input

## Repository license

### Context

`SPEC.md` requires the owner to choose a license and explicitly forbids an automatic selection. Until then, the repository uses “All rights reserved — license decision pending.”

### Why a decision is required

The choice determines who may use, modify, distribute, and combine the code and constrains compatible dependencies and contribution policy.

### Options

1. MIT — short and permissive with minimal conditions; no explicit patent grant.
2. Apache-2.0 — permissive with an explicit patent license and NOTICE obligations.
3. MPL-2.0 — file-level copyleft; modified covered files remain open while larger works may stay proprietary.
4. AGPL-3.0 — strong copyleft including network-use source obligations; materially restricts proprietary combinations.
5. Source-available or proprietary terms — customized control but not necessarily open source and requires careful legal drafting.

### Recommended option

Apache-2.0 if the goal is permissive open source with explicit patent terms. Obtain legal review before final adoption.

### Consequences

Permissive licenses maximize reuse but permit proprietary derivatives. Copyleft options require downstream source availability at different scopes. Custom terms reduce standard compatibility and increase legal/maintenance cost. Dependency license compatibility must be rechecked after selection.

### Work that can continue independently

Architecture, development, testing, dependency evaluation, and owner-authorized repository publication can continue under the temporary all-rights-reserved notice. Do not accept external contributions or publish a release until the license is selected.
