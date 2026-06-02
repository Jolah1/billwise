import { Pressable, StyleSheet, Text, View } from 'react-native';

import { useAuth } from '@/auth/context';

// Placeholder — the projects list lands in the next commit.
export default function Home() {
  const { accountId, signOut } = useAuth();
  return (
    <View style={s.container}>
      <Text style={s.title}>Signed in</Text>
      <Text style={s.subtle}>account {accountId}</Text>
      <Pressable style={s.button} onPress={signOut}>
        <Text style={s.buttonText}>Sign out</Text>
      </Pressable>
    </View>
  );
}

const s = StyleSheet.create({
  container: {
    flex: 1,
    padding: 24,
    justifyContent: 'center',
    alignItems: 'center',
    gap: 12,
  },
  title: { fontSize: 22, fontWeight: '600' },
  subtle: { color: '#666', fontSize: 12 },
  button: {
    marginTop: 24,
    paddingVertical: 12,
    paddingHorizontal: 24,
    borderRadius: 8,
    backgroundColor: '#208AEF',
  },
  buttonText: { color: 'white', fontWeight: '600' },
});
