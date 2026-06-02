import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import { useQueryClient } from '@tanstack/react-query';

import type { AuthResp } from '@/api/types';
import {
  clearStoredAuth,
  loadStoredAuth,
  saveStoredAuth,
} from './store';

type Status = 'loading' | 'signedIn' | 'signedOut';

interface AuthContextValue {
  status: Status;
  token: string | null;
  accountId: string | null;
  signIn: (resp: AuthResp) => Promise<void>;
  signOut: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<Status>('loading');
  const [token, setToken] = useState<string | null>(null);
  const [accountId, setAccountId] = useState<string | null>(null);
  const queryClient = useQueryClient();

  // Hydrate from secure-store on mount. Until this resolves, layouts
  // render nothing — that's the only safe way to avoid briefly flashing
  // the login screen for an already-signed-in user.
  useEffect(() => {
    let cancelled = false;
    loadStoredAuth().then((stored) => {
      if (cancelled) return;
      if (stored) {
        setToken(stored.token);
        setAccountId(stored.accountId);
        setStatus('signedIn');
      } else {
        setStatus('signedOut');
      }
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const signIn = useCallback(async (resp: AuthResp) => {
    await saveStoredAuth({ token: resp.token, accountId: resp.account_id });
    setToken(resp.token);
    setAccountId(resp.account_id);
    setStatus('signedIn');
  }, []);

  const signOut = useCallback(async () => {
    await clearStoredAuth();
    setToken(null);
    setAccountId(null);
    setStatus('signedOut');
    // Drop any cached account-scoped data so the next user doesn't see
    // the previous one's projects.
    queryClient.clear();
  }, [queryClient]);

  const value = useMemo<AuthContextValue>(
    () => ({ status, token, accountId, signIn, signOut }),
    [status, token, accountId, signIn, signOut],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used inside an AuthProvider');
  return ctx;
}

// Convenience for screens that are only mounted under (app) — they're
// already past the guard, so token is guaranteed.
export function useAuthToken(): { token: string; accountId: string } {
  const { token, accountId } = useAuth();
  if (!token || !accountId) {
    throw new Error(
      'useAuthToken called outside a signed-in route — check route guards',
    );
  }
  return { token, accountId };
}
