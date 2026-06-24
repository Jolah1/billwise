import { Platform } from 'react-native';
import * as SecureStore from 'expo-secure-store';

// Persisted as two separate keys (rather than a single JSON blob) so a
// partially written record can never look "valid" — either the token
// is fully there or auth is treated as signed out.
const TOKEN_KEY = 'billwise.auth.token';
const ACCOUNT_KEY = 'billwise.auth.accountId';

// expo-secure-store has no working web implementation in SDK 56, so we
// fall back to localStorage when running in the browser. This is for
// the dev preview target only — production ships native (iOS Keychain
// / Android Keystore) where SecureStore is genuinely secure.
const isWeb = Platform.OS === 'web';

async function getItem(key: string): Promise<string | null> {
  if (isWeb) return globalThis.localStorage?.getItem(key) ?? null;
  return SecureStore.getItemAsync(key);
}

async function setItem(key: string, value: string): Promise<void> {
  if (isWeb) {
    globalThis.localStorage?.setItem(key, value);
    return;
  }
  await SecureStore.setItemAsync(key, value);
}

async function deleteItem(key: string): Promise<void> {
  if (isWeb) {
    globalThis.localStorage?.removeItem(key);
    return;
  }
  await SecureStore.deleteItemAsync(key);
}

export interface StoredAuth {
  token: string;
  accountId: string;
}

export async function loadStoredAuth(): Promise<StoredAuth | null> {
  const [token, accountId] = await Promise.all([
    getItem(TOKEN_KEY),
    getItem(ACCOUNT_KEY),
  ]);
  if (!token || !accountId) return null;
  return { token, accountId };
}

export async function saveStoredAuth(auth: StoredAuth): Promise<void> {
  await Promise.all([
    setItem(TOKEN_KEY, auth.token),
    setItem(ACCOUNT_KEY, auth.accountId),
  ]);
}

export async function clearStoredAuth(): Promise<void> {
  await Promise.all([
    deleteItem(TOKEN_KEY),
    deleteItem(ACCOUNT_KEY),
  ]);
}
