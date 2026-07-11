# Performance Budgets

These are initial acceptance budgets to validate on representative Windows and Linux hardware. They are not claims about the current skeleton and may be revised only with measurements and rationale.

| Area | Initial budget |
| --- | --- |
| Idle CPU | median approximately 0%; no periodic work faster than 60 seconds unless an active, documented operation requires it |
| Idle disk/network | no continuous writes; zero application network requests in MVP |
| Cold usable window | target <= 2.5 s on representative SSD hardware, measured from launch to responsive shell |
| UI input response | target <= 100 ms for local navigation; no main-thread task > 50 ms in normal interaction |
| Search | p95 <= 300 ms for first page on 1,000,000 synthetic command records after warm-up |
| Pagination | <= 100 rendered list rows by default; no unbounded result set retained in React state |
| Import command | maximum 16 KiB after decoding; larger records rejected safely |
| Import memory | streaming; target peak working-set increase <= 64 MiB for a 1 GiB synthetic history file |
| Import batching | bounded batches, initially 500 records; tune from measurements |
| IPC request | default maximum 64 KiB unless a narrower command-specific limit is documented |
| Timeline metadata | maximum 16 KiB serialized per event with a versioned schema |
| Shutdown | owned background work cancelled and resources released within 5 s; interrupted work recoverable |
| Logs | 1 MiB active file, five 1 MiB archives, seven-day default retention; at most eight numeric context metrics per event and no background worker |

Benchmarks must include malformed and secret-heavy inputs so performance work never bypasses validation or redaction. Complexity is not added solely to meet a synthetic target without profiling.
