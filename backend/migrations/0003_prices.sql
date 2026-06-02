-- BillWise — two-layer price library.
-- global_prices: starter catalogue owned by the system.
-- account_price_overrides: sparse — only rows that differ from the global.
--
-- Resolution (lives in code, not in a view, so the join is explicit):
--
--   SELECT
--     g.id, g.description, g.unit,
--     COALESCE(o.rate_kobo, g.rate_kobo) AS rate_kobo,
--     (o.rate_kobo IS NOT NULL)          AS is_override
--   FROM global_prices g
--   LEFT JOIN account_price_overrides o
--     ON o.item_id = g.id AND o.account_id = $1
--   WHERE g.id = $2;

CREATE TABLE global_prices (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    description TEXT         NOT NULL,
    unit        TEXT         NOT NULL,
    rate_kobo   BIGINT       NOT NULL CHECK (rate_kobo >= 0),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE account_price_overrides (
    account_id UUID         NOT NULL REFERENCES account(id)       ON DELETE CASCADE,
    item_id    UUID         NOT NULL REFERENCES global_prices(id) ON DELETE CASCADE,
    rate_kobo  BIGINT       NOT NULL CHECK (rate_kobo >= 0),
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (account_id, item_id)
);
