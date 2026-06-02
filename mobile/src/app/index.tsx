import { Redirect } from 'expo-router';

import { useAuth } from '@/auth/context';

export default function Index() {
  const { status } = useAuth();
  if (status === 'loading') return null;
  return status === 'signedIn' ? (
    <Redirect href="/(app)" />
  ) : (
    <Redirect href="/(auth)/login" />
  );
}
