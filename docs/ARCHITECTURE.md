# BillWise Architecture

This document captures the load-bearing rules. New work must respect them.
If you find yourself fighting one, raise it and update this doc — don't
quietly route around it.

## Repository layout

```
backend/
  src/
    main.rs                bootstrap: config → pg pool → migrate → axum
    config.rs              env loading
    error.rs               AppError + IntoResponse
    domain/                pure types & rules — NO sqlx, NO axum imports
      money.rs             Kobo newtype + Display
      item.rs              ItemType enum + amount calc
      price.rs             PriceSource enum + ResolvedPrice
    db/                    sqlx queries; every account-scoped query
                           takes account_id explicitly
    auth/                  argon2 password hashing + JWT + extractor
    routes.rs              Router wiring
    handlers/              thin: extract → call domain/db → return JSON
  migrations/              sqlx migrations (0001_*, 0002_*, …)

mobile/
  app/                     expo-router file-based screens
  src/
    api/                   one typed client; the ONLY place fetch is called
    lib/money.ts           formatNaira — single source of truth for ₦
    auth/token.ts          SecureStore wrapper for the JWT
    types/api.ts           DTOs that mirror the backend
```

The layered backend split (routes / handlers / domain / db) exists so the
domain module stays pure and trivially unit-testable.

## Rule 1 — Money is integer kobo, never float

1 naira = 100 kobo. Money never leaves the integer track:

| Layer       | Representation                                            |
|-------------|-----------------------------------------------------------|
| Postgres    | `BIGINT` column named `*_kobo`                            |
| Rust        | `domain::money::Kobo(i64)` newtype                        |
| JSON (wire) | plain integer (serde serializes `Kobo` as `i64`)          |
| TypeScript  | `number` (asserted to be within `Number.MAX_SAFE_INTEGER`)|
| Display     | `Display` impl on `Kobo` on backend; `formatNaira(kobo)` in `mobile/src/lib/money.ts` |

**Kobo arithmetic rules** (enforced by the type):

- `Kobo + Kobo → Kobo`. You cannot add a Kobo to a plain integer or a float.
- Multiplying by a fractional quantity (e.g. `NUMERIC` from Postgres) returns
  a `Kobo` rounded to the **nearest** kobo (banker's rounding optional but
  not required for slice 1; use half-up).
- There is **no** division on `Kobo` in the slice — no per-unit-of-area
  computation yet.
- Formatting to "₦12,345.67" happens **only** at the presentation boundary
  (`Display` on the backend if we ever render server-side; `formatNaira` on
  mobile). No string formatting elsewhere — keeps decimal points and locale
  noise out of business logic.

## Rule 2 — Price resolution is two layers

Two tables:

- `global_prices(id, description, unit, rate_kobo)` — system-owned starter
  library.
- `account_price_overrides(account_id, item_id, rate_kobo)` — sparse;
  only rows where an account differs from the global default.

Resolution is a single `LEFT JOIN` returning whether the value was an
override:

```sql
SELECT
  g.id, g.description, g.unit,
  COALESCE(o.rate_kobo, g.rate_kobo) AS rate_kobo,
  (o.rate_kobo IS NOT NULL)          AS is_override
FROM global_prices g
LEFT JOIN account_price_overrides o
  ON o.item_id = g.id AND o.account_id = $1
WHERE g.id = $2;
```

The Rust side maps this to:

```rust
pub enum PriceSource { AccountOverride, GlobalDefault }

pub struct ResolvedPrice {
    pub item_id:     uuid::Uuid,
    pub description: String,
    pub unit:        String,
    pub rate:        Kobo,
    pub source:      PriceSource,
}
```

Carrying `source` through to the UI lets the mobile client badge custom
rates.

### Items snapshot their rate

`item.rate_kobo` is a **snapshot** taken at item-creation time. Items do not
foreign-key to `global_prices`. When the price-library UI lands, it will
*seed* the rate on item creation but the row owns its rate from then on.
This means changing a global price next year does not silently reprice
historical projects.

## Rule 3 — Amount is derived, not stored

An item's amount is `quantity × rate_kobo`, rounded to nearest kobo. The
server computes it; the client only displays it. A client-sent `amount`
field is ignored.

A project's total is the sum of its items' amounts, also computed on the
server and returned in `GET /projects/{id}`.

## Rule 4 — Multi-tenancy from day one

Every query that touches an account-owned table takes an `account_id` and
filters on it. No exceptions. The auth extractor yields `AccountId(Uuid)`;
handlers pass it straight through to the `db/` module. There is no
"current user" thread-local or implicit scope — passing the id explicitly
makes leaks obvious in code review.

Cascade-deletes are wired account → project → section → item so a future
"delete account" endpoint is one statement.

## Slice 1 scope — what is NOT here

- No PDF export
- No price-library UI (tables exist, resolution query exists, UI later)
- No rate build-up engine
- No soft-deletes
- No password reset / email verification
- No pagination on list endpoints (small data for slice 1)

When adding any of these, revisit this doc.
