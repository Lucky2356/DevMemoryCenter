# AGENTS.md

## Project

**Dev Recall** is a local-first desktop application for Windows and Linux that helps developers and system administrators restore the context of their work.

The application organizes local information about:

* development projects;
* terminal command history;
* work sessions;
* Git context;
* errors;
* notes;
* next steps;
* local Docker context;
* local ports and processes.

The application must remain private, secure, lightweight, understandable, and useful during everyday development.

The complete product requirements are defined in `SPEC.md`.

`SPEC.md` is the primary source of truth for product behavior, security requirements, architecture constraints, privacy, testing, performance, and scope.

---

# 1. Mandatory reading order

At the beginning of every work session, read the following files in this exact order:

1. `AGENTS.md`
2. `SPEC.md`
3. `TODO.md`
4. `PROJECT_STATE.md`
5. `NEEDS_USER_INPUT.md`, if it exists
6. `SECURITY_FINDINGS.md`, if it exists
7. relevant ADR documents
8. recent Git history
9. current Git status

Use:

```bash
git status
git log --oneline -10
```

Do not begin implementation before understanding the current state of the repository.

Do not assume that information from a previous agent session is still correct. Verify it against the code, tests, documentation, and Git history.

---

# 2. Instruction priority

When instructions conflict, use this priority order:

1. Explicit current instruction from the project owner
2. `SPEC.md`
3. `AGENTS.md`
4. Accepted ADR documents
5. `TODO.md`
6. `PROJECT_STATE.md`
7. Existing implementation

Do not silently resolve a serious contradiction.

Record important contradictions in `NEEDS_USER_INPUT.md`.

Continue independent work that does not depend on the unresolved decision.

---

# 3. Current technology stack

Use the stack defined in `SPEC.md`.

Current expected stack:

* Rust stable
* Safe Rust by default
* Tauri 2
* React
* TypeScript in strict mode
* Vite
* SQLite
* SQL migrations
* local-first architecture
* Windows and Linux support

Do not replace the selected stack without an explicit decision from the project owner.

Do not introduce a second backend language.

Do not add Electron.

Do not add a standalone web server unless it becomes an explicitly approved requirement.

Do not add cloud infrastructure in the MVP.

---

# 4. Main development principles

Always follow these principles:

* local-first;
* privacy by default;
* least privilege;
* deny by default;
* secure by design;
* minimal attack surface;
* deterministic behavior;
* explicit user consent;
* bounded resource usage;
* graceful cancellation;
* clear error handling;
* accessible interface;
* small reversible changes;
* test before commit;
* documentation matches actual behavior.

Prefer a smaller secure implementation over a larger unfinished implementation.

Prefer explicit code over hidden magic.

Prefer standard library functionality over unnecessary dependencies.

Prefer simple architecture over premature abstractions.

Do not sacrifice security, privacy, correctness, or recoverability to finish a task faster.

---

# 5. Scope control

Implement only functionality required by the current approved phase.

Do not expand the product scope without explicit instruction.

The MVP must not include:

* artificial intelligence;
* cloud synchronization;
* user registration;
* remote server administration;
* SSH connections;
* remote Docker;
* automatic execution of terminal commands;
* shell command execution from the UI;
* plugins downloaded from the internet;
* third-party marketplace;
* telemetry enabled by default;
* automatic crash report uploads;
* source code indexing;
* keylogging;
* screen recording;
* background clipboard reading;
* port scanning;
* network scanning;
* system administration with elevated privileges.

Interfaces for future functionality may be documented, but do not create unused complex abstractions merely for hypothetical features.

---

# 6. Work unit size

Work on one small, logically complete task at a time.

A task should normally:

* have a clear result;
* affect a limited set of files;
* be independently testable;
* leave the project buildable;
* be understandable in one Git commit;
* not mix unrelated refactoring and functionality.

Examples of acceptable work units:

* create the Rust workspace;
* add the initial SQLite migration;
* implement a `Project` domain entity;
* add project path validation;
* create the project list empty state;
* implement one PowerShell history parser;
* add secret redaction for JWT values;
* add cancellation to an import operation.

Examples of tasks that are too large:

* implement Terminal Memory;
* build the entire frontend;
* add all security features;
* complete the MVP;
* refactor the whole architecture.

Break large tasks into smaller steps before implementation.

---

# 7. Required workflow for every task

For each task:

1. Identify the exact requirement in `SPEC.md`.
2. Inspect related implementation and tests.
3. Define the smallest valid change.
4. Identify security and privacy implications.
5. Identify performance and resource implications.
6. Implement the change.
7. Add or update tests.
8. Run formatting.
9. Run linting.
10. Run relevant tests.
11. Run the build or type check.
12. Run relevant security checks.
13. Review the Git diff.
14. Update documentation.
15. Update `TODO.md`.
16. Update `PROJECT_STATE.md`.
17. Commit only if the repository is in a valid state.

Do not claim that a check passed unless it was actually executed successfully.

If a check cannot be run, record:

* which check was skipped;
* why it was skipped;
* how to run it later;
* what risk remains.

---

# 8. Repository state before work

Before modifying files:

* verify the current branch;
* inspect uncommitted changes;
* determine whether changes belong to a previous unfinished task;
* avoid overwriting user changes;
* avoid reverting changes you did not create;
* inspect recent commits.

If the working tree contains unrelated user changes, preserve them.

Do not use destructive Git commands to clean the repository.

Forbidden without explicit user instruction:

```bash
git reset --hard
git clean -fd
git clean -fdx
git checkout -- .
git restore .
git rebase
git push --force
```

Do not delete untracked files unless their origin and purpose are known.

---

# 9. Git rules

Use small, meaningful commits.

Each commit should contain one logical change.

Recommended commit prefixes:

```text
docs:
chore:
build:
ci:
feat:
fix:
refactor:
test:
perf:
security:
```

Examples:

```text
docs: add initial threat model
feat: add project path validation
security: redact bearer tokens from imported commands
test: add malformed PowerShell history fixtures
perf: stream terminal history import
```

Before committing:

```bash
git diff
git diff --staged
git status
```

Never commit:

* passwords;
* tokens;
* API keys;
* signing keys;
* certificates with private keys;
* personal command history;
* real user paths;
* production configuration;
* `.env` files with secrets;
* generated database files;
* crash dumps;
* diagnostic archives;
* dependency caches;
* build output unless specifically required.

Never push or publish without an explicit instruction from the project owner.

Do not create releases without explicit instruction.

Do not change repository visibility.

---

# 10. Branch strategy

Until another strategy is approved:

* use the current working branch;
* keep commits small;
* do not create branches automatically for every task;
* do not merge branches automatically;
* do not rebase shared history;
* do not force push.

If a new branch is clearly needed, record the recommendation in `NEEDS_USER_INPUT.md` unless the project owner has already authorized autonomous branch creation.

---

# 11. Rust rules

Use Safe Rust by default.

Avoid `unsafe`.

If `unsafe` becomes unavoidable:

* isolate it in the smallest possible module;
* add a `SAFETY` comment;
* document assumptions;
* add tests;
* update `SECURITY.md`;
* create or update an ADR;
* record why a safe alternative was insufficient.

Avoid in production code:

```rust
unwrap()
expect()
panic!()
todo!()
unimplemented!()
```

Exceptions are acceptable in tests or for statically guaranteed invariants, but explain non-obvious cases.

Use typed errors.

Do not expose raw internal errors directly to the UI.

Handle:

* invalid input;
* unavailable files;
* permission failures;
* corrupted data;
* cancelled operations;
* unsupported formats;
* missing external tools;
* database errors;
* migration failures.

Prefer ownership and borrowing over cloning.

Do not clone large collections merely to satisfy the borrow checker.

Avoid:

* unbounded `Vec` growth;
* unbounded caches;
* unbounded channels;
* detached tasks;
* blocking operations on async executors;
* unnecessary `Arc<Mutex<_>>`;
* lock held across `.await`;
* cyclic `Arc` references;
* global mutable state.

Every long-running operation must support cancellation.

Every spawned task must have an owner and lifecycle.

---

# 12. Async and concurrency rules

Use asynchronous code only where it provides a real benefit.

Do not make ordinary CPU-bound or short local operations async without reason.

For background jobs:

* use bounded channels;
* use backpressure;
* support cancellation;
* limit concurrency;
* release resources on shutdown;
* report progress;
* handle partial failure;
* prevent duplicate execution.

Do not create busy loops.

Do not poll frequently when an event-based mechanism exists.

If polling is necessary:

* document why;
* use a reasonable interval;
* pause when the application is inactive if possible;
* stop polling when the related feature is disabled;
* provide cancellation.

Do not hold database transactions or locks while waiting for unrelated I/O.

---

# 13. TypeScript and React rules

Use TypeScript strict mode.

Avoid `any`.

If `any` is unavoidable, isolate it and explain why.

Prefer:

* explicit domain types;
* discriminated unions;
* runtime validation for IPC input;
* small components;
* predictable state;
* accessible semantic HTML;
* cleanup in every effect that registers resources.

Do not use:

* `dangerouslySetInnerHTML`;
* `eval`;
* `new Function`;
* dynamic remote scripts;
* HTML from untrusted input;
* inline event handler strings;
* unsafe URL schemes.

Treat all data from Rust, SQLite, terminal history, Git, files, and processes as untrusted display data.

Escape and render it as text.

React effects must clean up:

* event listeners;
* timers;
* subscriptions;
* observers;
* pending requests where applicable.

Avoid storing very large result sets in component state.

Use pagination or virtualization for long lists.

---

# 14. Tauri security rules

Use minimal Tauri capabilities.

The frontend must not receive unrestricted access to:

* filesystem;
* shell;
* process execution;
* environment variables;
* database files;
* arbitrary paths;
* system commands.

Each IPC command must:

* have one clear purpose;
* accept a typed request;
* validate all fields;
* enforce size limits;
* validate paths;
* return a typed response;
* return sanitized errors;
* avoid exposing internal implementation details.

Do not implement a generic IPC endpoint such as:

```text
execute_command(command)
read_file(path)
write_file(path, data)
run_sql(query)
```

Create narrow domain-specific commands instead.

Examples:

```text
add_project(request)
list_projects(request)
preview_terminal_history_import(request)
cancel_import(request)
delete_project_record(request)
```

Remote content is forbidden.

Do not load scripts, styles, fonts, or application code from a CDN.

Do not enable the shell plugin in the MVP.

Do not enable development tools in production builds without explicit approval.

---

# 15. Database rules

All database access must use parameterized queries.

Never concatenate user input into SQL.

Use:

* migrations;
* transactions;
* foreign keys;
* constraints;
* indexes based on measured queries;
* bounded connection pools;
* consistent timestamps;
* versioned schemas.

Do not store:

* raw secrets;
* plaintext credentials;
* private keys;
* complete environment variable dumps;
* terminal output by default;
* source file contents;
* full diagnostic dumps.

Database migrations must be tested from:

* an empty database;
* the previous schema version;
* a representative populated database when relevant.

Do not edit an already released migration.

Create a new migration.

Before destructive schema changes:

* document the migration;
* assess data loss;
* create a recovery strategy;
* require explicit approval when user data could be lost.

---

# 16. File and path security

Treat every path as untrusted.

Before accessing a path:

* validate type and length;
* normalize where appropriate;
* canonicalize when required;
* detect symlinks;
* confirm the path is within an allowed scope;
* handle Windows-specific paths;
* handle UNC paths;
* handle invalid Unicode;
* handle files disappearing between checks and reads.

Do not recursively scan arbitrary directories without strict limits.

Apply limits to:

* directory depth;
* number of entries;
* total metadata processed;
* individual file size;
* total imported size;
* operation duration.

Exclude known heavy directories by default:

```text
.git/objects
node_modules
target
.venv
venv
vendor
dist
build
```

Never delete or modify files in a user project as part of removing a project from Dev Recall.

Removing a project from Dev Recall must only remove Dev Recall metadata unless the user explicitly performs a separately designed destructive action in a future version.

---

# 17. Terminal history rules

Terminal history is sensitive.

Before storing any imported command:

1. enforce a maximum input length;
2. normalize safely;
3. run secret redaction;
4. apply exclusion rules;
5. calculate a safe fingerprint if needed;
6. store only the redacted representation;
7. avoid logging the raw command.

Never create a database field intended to preserve the original unredacted command.

Do not temporarily persist unredacted commands to disk.

Do not include raw commands in:

* exceptions;
* traces;
* debug logs;
* test snapshots;
* crash reports;
* analytics;
* diagnostic bundles.

Fixtures must use fake credentials and obviously non-production data.

---

# 18. Secret redaction rules

The redaction system is a security boundary.

Changes to it require:

* unit tests;
* regression tests;
* property-based tests when appropriate;
* malformed input tests;
* performance tests for large input;
* false-positive review;
* false-negative review.

Use a regex engine resistant to catastrophic backtracking.

Apply limits to user-defined rules.

Never reveal a detected secret in an error message.

Redaction should happen as early as possible.

Prefer:

```text
raw input
→ size validation
→ redaction
→ domain processing
→ persistence
```

Avoid:

```text
raw input
→ logging
→ persistence
→ later redaction
```

---

# 19. Logging rules

Use structured logs.

Logs must have:

* timestamp;
* level;
* component;
* operation or correlation ID;
* sanitized context.

Logs must not contain:

* raw commands;
* secrets;
* authorization headers;
* cookies;
* credentials;
* source code;
* full environment variables;
* private user notes;
* complete file paths when unnecessary.

Prefer safe identifiers over raw values.

Log files must:

* rotate;
* have a size limit;
* have a retention period;
* be stored with appropriate permissions;
* be removable from the privacy settings.

Do not add remote log collection.

---

# 20. Privacy rules

No data collection starts before explicit onboarding consent.

Every data source must be separately controllable.

The user must be able to:

* view collected data;
* pause collection;
* disable a source;
* delete individual records;
* delete project records;
* delete all application data;
* configure retention;
* export data;
* review redaction settings.

Do not silently enable a new data source after an application update.

Do not send any user data over the network unless a future feature explicitly requires it and the user has given informed consent.

---

# 21. Performance rules

The application should consume minimal resources while idle.

Do not continuously:

* scan all projects;
* rescan terminal history;
* query Docker;
* inspect processes;
* write to SQLite;
* rebuild search indexes;
* perform network requests.

Use event-driven updates where practical.

Large operations must:

* stream data;
* use batches;
* limit memory;
* support cancellation;
* report progress;
* avoid blocking the UI;
* avoid one transaction per record;
* avoid loading all records at once.

For long lists use:

* pagination;
* cursor-based loading;
* virtualized rendering.

Every performance optimization must preserve correctness and security.

Measure before introducing complex optimization.

---

# 22. Resource and memory safety

Rust prevents many memory safety bugs, but it does not prevent all resource leaks.

Review all long-lived code for:

* cyclic `Arc` or `Rc`;
* retained channels;
* orphaned tasks;
* unclosed files;
* unreleased database connections;
* active filesystem watchers;
* timers without cancellation;
* caches without limits;
* React listeners without cleanup;
* large retained search results.

All caches must define:

* maximum size;
* eviction policy;
* invalidation strategy;
* lifecycle.

All background services must define:

* start;
* stop;
* cancellation;
* cleanup;
* error reporting.

Closing the application must terminate all owned work cleanly within a reasonable time.

---

# 23. Dependency rules

Before adding any dependency, evaluate:

* whether it is necessary;
* whether the standard library is sufficient;
* maintenance activity;
* release stability;
* security history;
* transitive dependencies;
* license;
* platform support;
* binary size impact;
* runtime impact;
* required permissions.

Do not add dependencies from arbitrary Git branches.

Prefer registry releases with lockfiles.

Do not add packages that execute unnecessary install scripts.

Do not add a dependency solely to avoid writing a small, clear, testable function.

Update `DEPENDENCIES.md` for security-critical dependencies.

After dependency changes run relevant audits.

---

# 24. Testing requirements

Every meaningful behavior change requires tests.

Use the appropriate level:

* unit test;
* integration test;
* component test;
* end-to-end test;
* property test;
* fuzz test;
* benchmark;
* security regression test.

A bug fix should normally include a test that fails before the fix and passes after it.

Security-sensitive parsers require malformed and adversarial fixtures.

Test:

* success paths;
* empty states;
* invalid input;
* oversized input;
* permission errors;
* cancellation;
* partial failure;
* duplicate data;
* corrupted data;
* restart recovery.

Do not delete or weaken tests to make CI pass.

Do not replace meaningful assertions with snapshots that hide behavior.

---

# 25. Required checks

Run checks appropriate to the current repository state.

Expected Rust checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets
cargo audit
cargo deny check
```

Expected frontend checks:

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test
npm run build
```

Use the actual package manager and scripts configured in the repository.

Do not invent commands that do not exist.

If the project uses another approved package manager, follow the lockfile:

* `package-lock.json` → npm;
* `pnpm-lock.yaml` → pnpm;
* `yarn.lock` → Yarn.

Do not create multiple frontend lockfiles.

For a focused small change, run focused tests during development, then run the broader required suite before commit.

---

# 26. Security checks

After security-sensitive changes, run appropriate checks such as:

* dependency audit;
* static analysis;
* malformed input tests;
* path traversal tests;
* symlink tests;
* XSS tests;
* SQL injection tests;
* redaction regression tests;
* oversized input tests;
* fuzz targets;
* permission tests.

Record newly found vulnerabilities in `SECURITY_FINDINGS.md`.

Do not postpone a known high-impact vulnerability while continuing feature development.

---

# 27. User interface rules

The interface must be:

* modern;
* calm;
* professional;
* accessible;
* keyboard-friendly;
* understandable without reading source code.

Every screen needs:

* loading state;
* empty state;
* normal state;
* error state;
* disabled state where relevant.

Dangerous actions need clear confirmation.

Do not rely solely on color for meaning.

Do not overload the dashboard.

Do not expose implementation terminology when a clear user-facing phrase exists.

Examples:

Prefer:

```text
Could not read PowerShell history.
The file is unavailable or access was denied.
```

Avoid:

```text
std::io::ErrorKind::PermissionDenied
```

Technical details may be available in an expandable diagnostic section after sanitization.

---

# 28. Localization rules

All user-facing strings must use the localization system.

Initial languages:

* English;
* Russian.

Do not hardcode user-facing text directly in reusable components.

Internal code, identifiers, comments, Git commits, and technical documentation should be in English unless a user-facing document requires translation.

Format dates, times, numbers, and durations according to locale.

---

# 29. Documentation rules

Update documentation together with implementation.

Documentation must reflect what is actually implemented.

Do not mark planned features as available.

Relevant files include:

* `README.md`;
* `SPEC.md`;
* `TODO.md`;
* `PROJECT_STATE.md`;
* `ARCHITECTURE.md`;
* `SECURITY.md`;
* `THREAT_MODEL.md`;
* `PRIVACY.md`;
* `DEPENDENCIES.md`;
* ADR documents;
* user documentation.

Create an ADR when making a significant architectural decision.

An ADR should contain:

* context;
* decision;
* considered alternatives;
* security implications;
* privacy implications;
* performance implications;
* consequences;
* status.

Do not rewrite accepted ADR history. Supersede it with a new ADR.

---

# 30. TODO.md rules

`TODO.md` is the executable project plan.

Tasks must be:

* small;
* ordered;
* testable;
* assigned to a phase;
* marked with status.

Recommended structure:

```markdown
# TODO

## Current phase

### In progress

- [ ] Task

### Ready

- [ ] Task
- [ ] Task

### Blocked

- [ ] Task — reason

### Completed

- [x] Task
```

Keep only one primary task in `In progress` unless parallel work is explicitly justified.

When completing a task:

* mark it complete;
* add discovered follow-up tasks;
* do not silently remove unfinished work;
* move blocked decisions to the blocked section.

---

# 31. PROJECT_STATE.md rules

Update `PROJECT_STATE.md` at the end of every meaningful work unit and before stopping when possible.

Use this structure:

```markdown
# Project State

## Current phase

## Last completed task

## Work in progress

## Completed

## Tests passed

## Checks not run

## Security checks passed

## Performance measurements

## Known issues

## Security findings

## Decisions required

## Next task

## Last stable commit

## Commands to verify
```

Be precise.

Do not write vague updates such as:

```text
Worked on project.
```

Write:

```text
Implemented PowerShell history line parser with a 16 KiB command limit.
Added JWT and Bearer token redaction tests.
```

`Last stable commit` must reference an actual commit when available.

---

# 32. NEEDS_USER_INPUT.md rules

Create or update `NEEDS_USER_INPUT.md` only for decisions that genuinely require the project owner.

Each question must contain:

```markdown
## Decision title

### Context

### Why a decision is required

### Options

1. Option A
2. Option B

### Recommended option

### Consequences

### Work that can continue independently
```

Do not use this file to avoid making ordinary engineering decisions.

Make reasonable, reversible implementation choices independently when they are already covered by `SPEC.md`.

---

# 33. SECURITY_FINDINGS.md rules

Create this file when a security issue is discovered.

Do not include real secrets or unnecessary exploit payloads.

Use:

```markdown
## Finding title

### Severity

### Affected component

### Description

### Impact

### Reproduction using safe test data

### Remediation

### Regression test

### Status
```

Severity categories:

* Critical
* High
* Medium
* Low
* Informational

Critical and High findings block further feature work until mitigated or explicitly accepted by the project owner.

---

# 34. Error handling

Errors must be explicit and recoverable where possible.

Do not silently ignore:

* database failures;
* filesystem failures;
* parser failures;
* migration failures;
* cancellation;
* data corruption;
* permission errors;
* background task failure.

Distinguish:

* user input error;
* unsupported environment;
* temporary operating system error;
* application defect;
* corrupted data;
* security rejection.

Provide the UI with a safe error code and localized message.

Log technical context only after sanitization.

---

# 35. Cancellation and shutdown

Every long-running operation must support cancellation.

Examples:

* terminal history import;
* project scanning;
* search indexing;
* retention cleanup;
* export;
* database maintenance.

On application shutdown:

* stop accepting new work;
* signal cancellation;
* wait for owned tasks within a bounded period;
* flush safe pending state;
* release database connections;
* release file handles;
* stop watchers;
* do not corrupt imports or migrations.

Test shutdown during active work.

---

# 36. Data deletion

Deletion must be scoped and explicit.

Supported deletion behavior must distinguish:

* remove a project from Dev Recall;
* delete project-related metadata;
* delete imported terminal records;
* delete a work session;
* delete all application data.

Never delete source project files.

Before deletion, show:

* what data will be deleted;
* whether it is recoverable;
* whether the original project files are affected.

Destructive database operations must use transactions where possible.

---

# 37. Export rules

Exported data must pass through privacy filters again.

Do not assume stored data is safe simply because it was previously redacted.

For CSV exports, protect against spreadsheet formula injection.

Do not export diagnostics or application database contents by default.

Exports must not include:

* encryption keys;
* internal credentials;
* private system tokens;
* unrestricted filesystem metadata.

---

# 38. Future multi-user compatibility

The MVP remains local and single-user.

Do not implement accounts, servers, or synchronization.

However:

* use stable IDs;
* keep domain ownership concepts separable;
* avoid global singleton assumptions in domain logic;
* avoid schemas that cannot support workspace ownership later;
* isolate persistence behind interfaces where useful;
* avoid embedding local filesystem paths into public identifiers.

Do not overengineer distributed synchronization before it is required.

Future compatibility is a design consideration, not current scope.

---

# 39. Future AI compatibility

Do not implement AI.

Do not add external AI SDKs.

Do not send commands, notes, or project metadata to external services.

Future AI integration must remain behind interfaces described in `SPEC.md`.

Current search and recommendations must be deterministic and local.

Do not name ordinary rule-based behavior as AI.

---

# 40. Prohibited shortcuts

Do not:

* disable security protections to make development easier;
* use hardcoded absolute user paths;
* store raw command history before redaction;
* commit temporary credentials;
* expose generic shell execution;
* expose unrestricted filesystem access;
* use administrator rights unnecessarily;
* use silent fallback to insecure storage;
* disable TypeScript strict mode;
* suppress all compiler or linter warnings;
* add broad allow permissions to Tauri;
* ignore failing tests;
* comment out broken tests;
* replace implementation with fake success responses;
* claim production readiness without evidence;
* publish automatically;
* push automatically;
* create paid infrastructure;
* use real user data in fixtures.

---

# 41. Handling incomplete work

Do not commit code that leaves the default branch obviously broken.

If work cannot be completed in the current session:

1. return the repository to the last stable state where practical;
2. keep incomplete code isolated;
3. document exact progress in `PROJECT_STATE.md`;
4. add remaining tasks to `TODO.md`;
5. record verification commands;
6. do not mark the task completed;
7. do not claim checks passed.

Avoid leaving partially applied migrations or generated files without explanation.

---

# 42. Resuming after interruption or usage limit

When work resumes after an interruption:

1. read all mandatory context files;
2. inspect Git status;
3. inspect the last ten commits;
4. inspect uncommitted changes;
5. run the minimum relevant validation;
6. compare `PROJECT_STATE.md` with actual code;
7. correct stale project state documentation;
8. resume the first unfinished task.

Do not restart the project from scratch.

Do not duplicate already completed work.

Do not assume the previous agent stopped at a clean boundary.

If there is an incomplete change, determine whether to complete or safely revert only that specific agent-created change.

Never use broad destructive Git commands.

---

# 43. Autonomous execution behavior

When operating autonomously:

* select the first ready task from `TODO.md`;
* work on only one logical task;
* validate it;
* update state;
* create one commit;
* continue with the next task only if the repository is stable and execution capacity remains.

If all ready tasks are complete:

* inspect the current phase definition;
* identify missing required work;
* add clear tasks to `TODO.md`;
* do not invent features outside `SPEC.md`.

If the MVP is complete:

* run the full validation suite;
* perform a documentation consistency review;
* record remaining limitations;
* do not publish or release automatically.

---

# 44. Commit gate

A commit is allowed only when:

* the change has a clear purpose;
* relevant tests pass;
* formatting passes;
* linting passes;
* type checking passes;
* the build is not knowingly broken;
* security implications were reviewed;
* sensitive data was reviewed;
* documentation was updated;
* `TODO.md` was updated;
* `PROJECT_STATE.md` was updated.

If a non-critical check cannot run because the environment lacks a tool, a commit may be made only when:

* the limitation is documented;
* all available relevant checks pass;
* the code is not knowingly broken;
* verification instructions are recorded.

Do not commit a known security regression.

---

# 45. Definition of task completion

A task is complete only when:

* implementation exists;
* tests exist or an explicit reason is documented;
* normal behavior works;
* invalid input is handled;
* loading state exists where applicable;
* empty state exists where applicable;
* error state exists where applicable;
* cancellation exists for long operations;
* privacy implications are handled;
* security implications are handled;
* accessibility was considered;
* documentation is current;
* all relevant checks pass;
* project state is updated.

Writing code alone does not complete a task.

---

# 46. First agent run

On the first agent run after this file is added:

1. Read `SPEC.md` completely.
2. Inspect the repository.
3. Do not assume the toolchain is installed.
4. Verify available versions of:

   * Rust;
   * Cargo;
   * Node.js;
   * package manager;
   * Tauri prerequisites;
   * Git.
5. Do not install system-wide dependencies without explicit approval.
6. Create the initial planning and documentation files required by `SPEC.md`.
7. Prepare the initial architecture and threat model.
8. Create at least five ADR documents.
9. Create `TODO.md`.
10. Create `PROJECT_STATE.md`.
11. Create `NEEDS_USER_INPUT.md` only if actual owner decisions are required.
12. Create the smallest compilable application skeleton only after the planning documents exist.
13. Run available checks.
14. Record missing environment prerequisites.
15. Commit the initial result only when stable.

The first run must not attempt to implement the full MVP.

---

# 47. Expected first ADRs

Prepare at least these ADRs:

```text
ADR-0001: Rust, Tauri, React, and TypeScript stack
ADR-0002: Local-first application architecture
ADR-0003: SQLite persistence strategy
ADR-0004: Terminal history import and shell integration boundaries
ADR-0005: Secret redaction before persistence
ADR-0006: Tauri capability and IPC security model
ADR-0007: Background task lifecycle and cancellation
ADR-0008: Future multi-user compatibility without cloud MVP
```

ADR numbering and names may be adjusted to match repository conventions.

---

# 48. Expected first project phases

The initial `TODO.md` should be organized approximately as:

```text
Phase 0 — Research and architecture
Phase 1 — Application foundation
Phase 2 — Local projects
Phase 3 — Terminal Memory import
Phase 4 — Search
Phase 5 — Work sessions
Phase 6 — Timeline
Phase 7 — Favorites
Phase 8 — Privacy Center
Phase 9 — Optional read-only local context
Phase 10 — Security, performance, and release hardening
```

Do not start a later phase while blocking requirements from the current phase remain unresolved.

---

# 49. Final response after an autonomous run

At the end of an agent run, provide a concise report containing:

* completed task;
* files created or changed;
* tests and checks run;
* security checks run;
* commit created;
* known limitations;
* decisions required;
* exact next task.

Do not report low-level activity that is already visible in logs.

Do not claim the entire project is complete unless every MVP requirement and completion criterion has been verified.

---

# 50. Core invariant

At all times preserve this invariant:

> Dev Recall must never become a hidden surveillance tool, an unrestricted command execution tool, or a storage location for unredacted secrets.

If a proposed feature violates this invariant, do not implement it.

Document the conflict and request a product decision.

# 51. Autonomous planning

Before starting a task:

- verify whether the task is already completed;
- avoid duplicating existing functionality;
- prefer completing partially finished work before starting new work;
- never work on two unrelated tasks simultaneously.

When multiple tasks are available:

- choose the earliest task in TODO.md;
- do not skip phases;
- complete current phase before starting another one.

When a phase is complete:

- verify SPEC.md requirements for that phase;
- add missing tasks if necessary;
- only then continue.

Always leave the repository in a releasable state.
