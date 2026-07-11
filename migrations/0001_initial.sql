-- Immutable initial schema. Add future changes as new migration files.
CREATE TABLE owners (
    id TEXT PRIMARY KEY NOT NULL
        CHECK (length(id) = 36),
    owner_type TEXT NOT NULL
        CHECK (length(owner_type) BETWEEN 1 AND 32),
    created_at_unix_ms INTEGER NOT NULL
        CHECK (created_at_unix_ms >= 0)
) STRICT;

CREATE UNIQUE INDEX owners_single_local_owner
    ON owners(owner_type)
    WHERE owner_type = 'local';
