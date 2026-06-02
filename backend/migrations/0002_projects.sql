-- BillWise — projects, sections, items.
-- Standard BOQ structural elements: a project has sections, sections have
-- items, each item has a unit rate and a quantity. Amount is DERIVED in
-- Rust from quantity * rate_kobo — never stored.

CREATE TYPE item_type AS ENUM (
    'measured',          -- normal measured work
    'provisional_sum',   -- lump sum allowance for unspecified work
    'pc_sum',            -- prime-cost sum (named supplier component)
    'prime_cost'         -- alias used by some quantity surveyors
);

CREATE TABLE project (
    id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id   UUID         NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    title        TEXT         NOT NULL,
    client       TEXT         NOT NULL,
    location     TEXT         NOT NULL,
    project_date DATE         NOT NULL,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX project_account_id_idx ON project(account_id);

CREATE TABLE section (
    id         UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID         NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    name       TEXT         NOT NULL,
    sort_order INTEGER      NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
CREATE INDEX section_project_id_idx ON section(project_id, sort_order);

CREATE TABLE item (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    section_id  UUID          NOT NULL REFERENCES section(id) ON DELETE CASCADE,
    description TEXT          NOT NULL,
    quantity    NUMERIC(18,4) NOT NULL CHECK (quantity >= 0),
    unit        TEXT          NOT NULL,
    rate_kobo   BIGINT        NOT NULL CHECK (rate_kobo >= 0),
    item_type   item_type     NOT NULL,
    sort_order  INTEGER       NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);
CREATE INDEX item_section_id_idx ON item(section_id, sort_order);
