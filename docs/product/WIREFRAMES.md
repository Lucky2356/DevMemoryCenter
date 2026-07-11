# Text Wireframes

These are planning artifacts, not implemented screens.

## Onboarding

```text
+--------------------------------------------------------------+
| Dev Recall                                      Step 1 of 4  |
| Local context stays on this device.                          |
|                                                              |
| [ ] I understand terminal history may contain secrets.       |
| [ ] Enable project metadata (preview first)                   |
| [ ] Enable a shell source (preview first)                     |
|                                                              |
| What is never done: command execution, cloud upload,          |
| keylogging, screen capture, background clipboard reading.     |
|                                           [Back] [Continue]   |
+--------------------------------------------------------------+
```

## Application shell and overview

```text
+------------------+-------------------------------------------+
| Dev Recall       | Overview                                  |
| Overview         | [No active session] [Start session]       |
| Projects         |                                           |
| Timeline         | Recent projects        Next steps         |
| Terminal Memory  | [Empty/loading/error/list state]           |
| Sessions         |                                           |
| Favorites        | Recent safe activity   Data/source status |
| Privacy & Data   | [Bounded lists with View all]              |
| Settings         |                                           |
+------------------+-------------------------------------------+
```

## Project detail

```text
+--------------------------------------------------------------+
| Project name                           [Refresh] [Open folder]|
| path (selectable text) | type | branch | clean/changed label  |
| [Summary] [Timeline] [Sessions] [Commands] [Favorites]        |
|                                                              |
| Safe metadata / empty / loading / recoverable error state     |
|                                                              |
| [Archive record] [Remove from Dev Recall...]                  |
| Source project files are never deleted.                       |
+--------------------------------------------------------------+
```

## Terminal import preview

```text
+--------------------------------------------------------------+
| Import terminal history                         Step 2 of 3  |
| Source: PowerShell        File: user-approved source          |
| Limits and exclusions: [Review]                               |
|                                                              |
| Preview (redacted; raw commands are not persisted)            |
| 12:03  curl ... token=<REDACTED>                    [Include] |
| 12:08  cargo test                                   [Include] |
|                                                              |
| 2 shown | 1 secret hidden | 0 rejected                        |
|                                  [Cancel] [Confirm import]    |
+--------------------------------------------------------------+
```

## Search and timeline

```text
+--------------------------------------------------------------+
| Search [________________________] [Filters]                    |
| Project: All | Type: All | Date: Any | Status: Any            |
|                                                              |
| Results 1-50 (keyboard navigable, virtualized/paginated)      |
| > safe text result                       project • timestamp   |
|   safe text result                       project • timestamp   |
|                                        [Previous] [Next]      |
+--------------------------------------------------------------+
```

## Privacy & Data

```text
+--------------------------------------------------------------+
| Privacy & Data                                               |
| Collection: Paused                         [Resume]            |
| Sources: PowerShell Off | Bash Off | Project metadata On      |
| Storage: records / database size / last import                |
| Redaction rules [Review]   Retention [30 days v]              |
|                                                              |
| [Export with preview] [Delete selected data...] [Full reset]  |
| Every destructive action explains scope and recoverability.   |
+--------------------------------------------------------------+
```
