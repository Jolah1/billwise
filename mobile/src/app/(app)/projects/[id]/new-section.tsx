import { useState } from 'react';
import {
  ActivityIndicator,
  KeyboardAvoidingView,
  Platform,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  View,
} from 'react-native';
import { useLocalSearchParams, useRouter } from 'expo-router';

import { ApiError } from '@/api/client';
import { useCreateSection } from '@/queries/sections';

export default function NewSection() {
  const router = useRouter();
  const { id } = useLocalSearchParams<{ id: string }>();
  const [name, setName] = useState('');

  const m = useCreateSection(id);

  const submit = () => {
    m.mutate(
      { name },
      {
        onSuccess: () => router.back(),
      },
    );
  };

  const errorMessage =
    m.error instanceof ApiError
      ? m.error.message
      : m.error
        ? 'Could not save section'
        : null;

  return (
    <KeyboardAvoidingView
      style={s.fill}
      behavior={Platform.OS === 'ios' ? 'padding' : undefined}
    >
      <ScrollView
        contentContainerStyle={s.container}
        keyboardShouldPersistTaps="handled"
      >
        <View style={s.field}>
          <Text style={s.label}>Name</Text>
          <TextInput
            style={s.input}
            value={name}
            onChangeText={setName}
            placeholder="e.g. Substructure"
            autoFocus
          />
        </View>

        {errorMessage ? <Text style={s.error}>{errorMessage}</Text> : null}

        <Pressable
          style={({ pressed }) => [
            s.button,
            (m.isPending || pressed) && s.buttonPressed,
          ]}
          onPress={submit}
          disabled={m.isPending}
        >
          {m.isPending ? (
            <ActivityIndicator color="white" />
          ) : (
            <Text style={s.buttonText}>Create section</Text>
          )}
        </Pressable>
      </ScrollView>
    </KeyboardAvoidingView>
  );
}

const s = StyleSheet.create({
  fill: { flex: 1, backgroundColor: 'white' },
  container: { padding: 20, gap: 16 },
  field: { gap: 6 },
  label: { fontSize: 14, fontWeight: '600' },
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
});
