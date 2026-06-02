import * as SecureStore from 'expo-secure-store';

// Persisted as two separate keys (rather than a single JSON blob) so a
// partially written record can never look "valid" — either the token
// is fully there or auth is treated as signed out.
const TOKEN_KEY = 'billwise.auth.token';
const ACCOUNT_KEY = 'billwise.auth.accountId';

export interface StoredAuth {
  token: string;
  accountId: string;
}

export async function loadStoredAuth(): Promise<StoredAuth | null> {
  const [token, accountId] = await Promise.all([
    SecureStore.getItemAsync(TOKEN_KEY),
    SecureStore.getItemAsync(ACCOUNT_KEY),
  ]);
  if (!token || !accountId) return null;
  return { token, accountId };
}

export async function saveStoredAuth(auth: StoredAuth): Promise<void> {
  await Promise.all([
    SecureStore.setItemAsync(TOKEN_KEY, auth.token),
    SecureStore.setItemAsync(ACCOUNT_KEY, auth.accountId),
  ]);
}

export async function clearStoredAuth(): Promise<void> {
  await Promise.all([
    SecureStore.deleteItemAsync(TOKEN_KEY),
    SecureStore.deleteItemAsync(ACCOUNT_KEY),
  ]);
}
