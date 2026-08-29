CREATE TABLE practices (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX practices_created_at_idx ON practices (created_at DESC);

CREATE TABLE sources (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT sources_url_not_empty CHECK (url <> '')
);

CREATE TABLE practice_sources (
    practice_id UUID NOT NULL REFERENCES practices (id) ON DELETE CASCADE,
    source_id UUID NOT NULL REFERENCES sources (id) ON DELETE CASCADE,
    PRIMARY KEY (practice_id, source_id)
);

CREATE INDEX practice_sources_source_id_idx ON practice_sources (source_id);

CREATE TABLE experiences (
    id UUID PRIMARY KEY,
    practice_id UUID NOT NULL REFERENCES practices (id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT experiences_note_valid CHECK (
        note IS NULL OR (
            char_length(note) <= 100
            AND lower(note) NOT LIKE '%http://%'
            AND lower(note) NOT LIKE '%https://%'
            AND lower(note) NOT LIKE '%www.%'
        )
    )
);

CREATE INDEX experiences_practice_id_idx ON experiences (practice_id);
CREATE INDEX experiences_user_id_idx ON experiences (user_id);

CREATE TABLE listing_requests (
    id UUID PRIMARY KEY,
    source_url TEXT NOT NULL,
    user_id UUID NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT listing_requests_source_url_unique UNIQUE (source_url),
    CONSTRAINT listing_requests_source_url_not_empty CHECK (source_url <> ''),
    CONSTRAINT listing_requests_status_valid CHECK (
        status IN ('pending', 'accepted', 'rejected')
    )
);
