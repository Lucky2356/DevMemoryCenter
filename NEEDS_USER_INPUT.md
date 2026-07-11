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

Architecture, private local development, testing, and dependency evaluation can continue under the temporary all-rights-reserved notice. Publishing or accepting external contributions should wait for the decision.

## GitHub publication of the required future-AI document

### Context

The current owner instruction says never to upload anything related to AI to GitHub. At the same time, `SPEC.md` explicitly requires the repository document `FUTURE_AI.md`, and that file is already part of the local Git history from the first-run commit. No push has been performed.

### Why a decision is required

Pushing the existing history would publish `FUTURE_AI.md`, while removing or rewriting it would conflict with the current `SPEC.md`. The intended meaning of “anything related to AI” must be clarified before publication.

### Options

1. Treat the restriction as prohibiting AI tooling, generated attribution, integrations, and co-author metadata, while retaining the required product-scope document.
2. Remove the future-AI document and explicitly amend `SPEC.md` before any push.
3. Keep the repository local and do not publish until the product requirements are revised.

### Recommended option

Option 1, if the intent is to present normal human-maintained project history without AI tooling or attribution. The document itself prohibits AI in the MVP and records a product boundary rather than an integration.

### Consequences

Option 1 preserves the current specification but publishes a document whose title references AI. Option 2 changes the product documentation contract and requires a deliberate specification update. Option 3 blocks all GitHub publication but does not block local development.

### Work that can continue independently

All local implementation, testing, documentation, and commits can continue. Do not push, publish, create releases, or rewrite Git history until this is clarified.
