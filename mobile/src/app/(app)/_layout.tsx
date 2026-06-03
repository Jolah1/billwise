import { Redirect, Stack } from 'expo-router';

import { useAuth } from '@/auth/context';

export default function AppLayout() {
  const { status } = useAuth();
  if (status === 'loading') return null;
  if (status === 'signedOut') return <Redirect href="/(auth)/login" />;
  return (
    <Stack>
      <Stack.Screen name="index" options={{ title: 'Projects' }} />
      <Stack.Screen
        name="new-project"
        options={{ presentation: 'modal', title: 'New project' }}
      />
      <Stack.Screen name="projects/[id]" options={{ headerShown: false }} />
    </Stack>
  );
}
