import { Redirect, Stack } from 'expo-router';

import { useAuth } from '@/auth/context';

export default function AuthLayout() {
  const { status } = useAuth();
  if (status === 'loading') return null;
  if (status === 'signedIn') return <Redirect href="/(app)" />;
  return <Stack screenOptions={{ headerShown: false }} />;
}
