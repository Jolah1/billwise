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
import { useRouter } from 'expo-router';

import { ApiError } from '@/api/client';
import { useCreateProject } from '@/queries/projects';

export default function NewProject() {
  const router = useRouter();
  const [title, setTitle] = useState('');
  const [client, setClient] = useState('');
  const [location, setLocation] = useState('');
  const [projectDate, setProjectDate] = useState(todayISO());

  const m = useCreateProject();

  const submit = () => {
    m.mutate(
      { title, client, location, project_date: projectDate },
      {
        onSuccess: () => router.back(),
      },
    );
  };

  const errorMessage =
    m.error instanceof ApiError
      ? m.error.message
      : m.error
        ? 'Could not save project'
        : null;

  return (
    <KeyboardAvoidingView
      style={s.fill}
      behavior={Platform.OS === 'ios' ? 'padding' : undefined}
    >
      <ScrollView contentContainerStyle={s.container} keyboardShouldPersistTaps="handled">
        <Field label="Title">
          <TextInput
            style={s.input}
            value={title}
            onChangeText={setTitle}
            placeholder="e.g. Lekki Phase 1 villa"
          />
        </Field>

        <Field label="Client">
          <TextInput
            style={s.input}
            value={client}
            onChangeText={setClient}
            placeholder="e.g. Acme Ltd"
          />
        </Field>

        <Field label="Location">
          <TextInput
            style={s.input}
            value={location}
            onChangeText={setLocation}
            placeholder="e.g. Lagos"
          />
        </Field>

        <Field label="Project date" hint="YYYY-MM-DD">
          <TextInput
            style={s.input}
            value={projectDate}
            onChangeText={setProjectDate}
            placeholder="2026-06-02"
            autoCapitalize="none"
            autoCorrect={false}
          />
        </Field>

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
            <Text style={s.buttonText}>Create project</Text>
          )}
        </Pressable>
      </ScrollView>
    </KeyboardAvoidingView>
  );
}

function Field({
  label,
  hint,
  children,
}: {
  label: string;
  hint?: string;
  children: React.ReactNode;
}) {
  return (
    <View style={s.field}>
      <Text style={s.label}>
        {label}
        {hint ? <Text style={s.hint}>  {hint}</Text> : null}
      </Text>
      {children}
    </View>
  );
}

function todayISO() {
  const d = new Date();
  const yyyy = d.getFullYear();
  const mm = String(d.getMonth() + 1).padStart(2, '0');
  const dd = String(d.getDate()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd}`;
}

const s = StyleSheet.create({
  fill: { flex: 1, backgroundColor: 'white' },
  container: { padding: 20, gap: 16 },
  field: { gap: 6 },
  label: { fontSize: 14, fontWeight: '600' },
  hint: { fontSize: 12, fontWeight: '400', color: '#999' },
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
