import { Redirect, Stack } from 'expo-router';

import { useAuth } from '@/auth/context';

export default function AppLayout() {
  const { status } = useAuth();
  if (status === 'loading') return null;
  if (status === 'signedOut') return <Redirect href="/(auth)/login" />;
  return <Stack />;
}
