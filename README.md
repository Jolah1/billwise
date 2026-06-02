# BillWise

A mobile app for generating professional Bills of Quantities (BOQ) for the
Nigerian construction market.

This repo is a monorepo:

```
backend/   Rust + axum + sqlx + Postgres
mobile/    Expo + React Native + TypeScript (expo-router)
docs/      Architecture & design notes
```

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the non-negotiable
domain rules (money-as-integer-kobo, two-layer price resolution,
account-scoped multi-tenancy).

## Prerequisites

- Rust (1.80+) — install via [rustup](https://rustup.rs)
- Docker + Docker Compose (or any local Postgres 14+)
- Node.js 20+ and npm
- [`sqlx-cli`](https://crates.io/crates/sqlx-cli) for running migrations:

  ```sh
  cargo install sqlx-cli --no-default-features --features rustls,postgres
  ```

## 1. Start Postgres

```sh
docker compose up -d postgres
```

This binds `localhost:5432` with database `billwise` / user `billwise` /
password `billwise`.

## 2. Backend

```sh
cd backend
cp .env.example .env             # then edit JWT_SECRET for non-trivial use
export $(grep -v '^#' .env | xargs)   # load env into shell (or use direnv)

sqlx migrate run                 # apply migrations from backend/migrations/
cargo test                       # run domain unit tests
cargo run                        # serve on $BIND_ADDR (default 0.0.0.0:3000)
```

Smoke test:

```sh
# health
curl -s http://localhost:3000/health

# register
curl -s -X POST http://localhost:3000/auth/register \
  -H 'content-type: application/json' \
  -d '{"email":"qs@example.com","password":"correct horse battery staple"}'

# login -> JWT
TOKEN=$(curl -s -X POST http://localhost:3000/auth/login \
  -H 'content-type: application/json' \
  -d '{"email":"qs@example.com","password":"correct horse battery staple"}' \
  | jq -r .token)

# create a project
curl -s -X POST http://localhost:3000/projects \
  -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"title":"Lekki 3-bed","client":"Acme","location":"Lagos","project_date":"2026-07-01"}'
```

(Endpoints land in the slices following the foundation; see task list.)

## 3. Mobile

```sh
cd mobile
cp .env.example .env             # set EXPO_PUBLIC_API_BASE_URL
npm install
npx expo start
```

Then open the QR code in Expo Go on your phone, or press `i` / `a` for the
iOS / Android simulator.

## Project status

This is the first vertical slice: auth → create project → add sections &
items → see project total in ₦. PDF export, the price-library UI, and the
rate build-up engine come later.
