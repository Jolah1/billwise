# BillWise mobile

Expo + React Native + TypeScript app for generating Bills of Quantities
(BOQ) for the Nigerian construction market. Pairs with the Rust backend
in `../backend`.

## Develop

```bash
npm install
npm start              # Metro bundler — open in Expo Go / simulator
npm run ios            # iOS simulator (macOS only)
npm run android        # Android emulator
```

## Configuration

Point the app at a backend with `EXPO_PUBLIC_API_URL`:

```bash
EXPO_PUBLIC_API_URL=http://192.168.1.10:3000 npm start
```

Defaults to `http://localhost:3000` (only works on the iOS simulator —
real devices need your machine's LAN address).

## Layout

- `src/app/` — expo-router file-based routes. `(auth)` group is the
  signed-out world; `(app)` group requires a valid JWT and routes back
  to `(auth)/login` when there isn't one.
- `src/api/` — typed fetch wrappers for each backend endpoint plus the
  DTO types.
- `src/auth/` — JWT storage (expo-secure-store) and the `AuthProvider`
  context.
- `src/money/` — single source of truth for kobo formatting. Components
  never format kobo themselves.
- `src/queries/` — React Query hooks; one file per backend resource.

See `docs/ARCHITECTURE.md` at the repo root for the domain rules
(integer kobo, server-derived totals, multi-tenancy).
