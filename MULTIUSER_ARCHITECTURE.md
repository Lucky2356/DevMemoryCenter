# Future Multi-user Architecture

The MVP is local and single-user. It has no registration, server, cloud sync, sharing, or network authorization. Stable IDs, `owner_id`, `workspace_id` where appropriate, and persistence ports prevent domain logic from depending on a global singleton.

A future design requires a separate approved phase covering tenant/workspace isolation; owner/admin/member/viewer RBAC; object-level authorization on every operation; tamper-evident audit trails; device registration/revocation; authenticated encrypted transport; per-tenant key and secret separation; zero-trust service boundaries; sessions and rate limits; sync conflicts and deletion tombstones; backup/recovery; exports, retention, account deletion, and GDPR-like rights. None of these are represented by fake network abstractions in MVP code.
