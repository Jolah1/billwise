// EXPO_PUBLIC_API_URL is the only env var the bundler exposes to the
// app at runtime. Defaulting to localhost works for the iOS simulator
// but real devices need the dev machine's LAN address — set the var
// explicitly when starting Metro.

export const API_BASE_URL: string =
  process.env.EXPO_PUBLIC_API_URL ?? 'http://localhost:3000';
