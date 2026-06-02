import { useState } from 'react';
import {
  ActivityIndicator,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  StyleSheet,
  Text,
  TextInput,
  View,
} from 'react-native';
import { Link } from 'expo-router';
import { useMutation } from '@tanstack/react-query';

import { login } from '@/api/auth';
import { ApiError } from '@/api/client';
import { useAuth } from '@/auth/context';

export default function Login() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const { signIn } = useAuth();

  const m = useMutation({
    mutationFn: login,
    onSuccess: (resp) => signIn(resp),
  });

  const errorMessage =
    m.error instanceof ApiError
      ? m.error.message
      : m.error
        ? 'Could not reach the server'
        : null;

  return (
    <KeyboardAvoidingView
      style={s.fill}
      behavior={Platform.OS === 'ios' ? 'padding' : undefined}
    >
      <View style={s.container}>
        <Text style={s.title}>BillWise</Text>
        <Text style={s.subtitle}>Sign in to your account</Text>

        <TextInput
          style={s.input}
          value={email}
          onChangeText={setEmail}
          placeholder="Email"
          keyboardType="email-address"
          autoCapitalize="none"
          autoComplete="email"
          textContentType="emailAddress"
        />
        <TextInput
          style={s.input}
          value={password}
          onChangeText={setPassword}
          placeholder="Password"
          secureTextEntry
          autoComplete="current-password"
          textContentType="password"
        />

        {errorMessage ? <Text style={s.error}>{errorMessage}</Text> : null}

        <Pressable
          style={({ pressed }) => [
            s.button,
            (m.isPending || pressed) && s.buttonPressed,
          ]}
          onPress={() => m.mutate({ email, password })}
          disabled={m.isPending}
        >
          {m.isPending ? (
            <ActivityIndicator color="white" />
          ) : (
            <Text style={s.buttonText}>Sign in</Text>
          )}
        </Pressable>

        <Link href="/(auth)/register" style={s.link}>
          Need an account? Register
        </Link>
      </View>
    </KeyboardAvoidingView>
  );
}

const s = StyleSheet.create({
  fill: { flex: 1 },
  container: { flex: 1, padding: 24, justifyContent: 'center', gap: 12 },
  title: { fontSize: 32, fontWeight: '700' },
  subtitle: { fontSize: 16, color: '#666', marginBottom: 16 },
  input: {
    borderWidth: 1,
    borderColor: '#ccc',
    paddingVertical: 12,
    paddingHorizontal: 14,
    borderRadius: 10,
    fontSize: 16,
  },
  button: {
    backgroundColor: '#208AEF',
    paddingVertical: 14,
    borderRadius: 10,
    alignItems: 'center',
    marginTop: 8,
  },
  buttonPressed: { opacity: 0.8 },
  buttonText: { color: 'white', fontSize: 16, fontWeight: '600' },
  error: { color: '#c0392b' },
  link: { color: '#208AEF', textAlign: 'center', marginTop: 16 },
});
