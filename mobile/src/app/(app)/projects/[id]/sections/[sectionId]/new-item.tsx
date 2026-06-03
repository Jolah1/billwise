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
import type { ItemType } from '@/api/types';
import { parseNairaToKobo } from '@/money/format';
import { useCreateItem } from '@/queries/items';

const ITEM_TYPES: { value: ItemType; label: string }[] = [
  { value: 'measured',        label: 'Measured' },
  { value: 'provisional_sum', label: 'Prov. sum' },
  { value: 'pc_sum',          label: 'PC sum' },
  { value: 'prime_cost',      label: 'Prime cost' },
];

export default function NewItem() {
  const router = useRouter();
  const { id, sectionId } = useLocalSearchParams<{
    id: string;
    sectionId: string;
  }>();

  const [description, setDescription] = useState('');
  const [unit, setUnit] = useState('');
  const [quantity, setQuantity] = useState('');
  const [rate, setRate] = useState('');
  const [itemType, setItemType] = useState<ItemType>('measured');
  const [localError, setLocalError] = useState<string | null>(null);

  const m = useCreateItem(id, sectionId);

  const submit = () => {
    setLocalError(null);
    const rateKobo = parseNairaToKobo(rate);
    if (rateKobo === null) {
      setLocalError('Rate must be a number with up to 2 decimal places');
      return;
    }
    if (!/^\d+(\.\d+)?$/.test(quantity.trim())) {
      setLocalError('Quantity must be a non-negative number');
      return;
    }
    m.mutate(
      {
        description,
        unit,
        quantity: quantity.trim(),
        rate_kobo: rateKobo,
        item_type: itemType,
      },
      { onSuccess: () => router.back() },
    );
  };

  const remoteError =
    m.error instanceof ApiError
      ? m.error.message
      : m.error
        ? 'Could not save item'
        : null;
  const errorMessage = localError ?? remoteError;

  return (
    <KeyboardAvoidingView
      style={s.fill}
      behavior={Platform.OS === 'ios' ? 'padding' : undefined}
    >
      <ScrollView
        contentContainerStyle={s.container}
        keyboardShouldPersistTaps="handled"
      >
        <Field label="Description">
          <TextInput
            style={s.input}
            value={description}
            onChangeText={setDescription}
            placeholder="e.g. 225mm thick concrete blockwork"
            multiline
          />
        </Field>

        <Field label="Unit">
          <TextInput
            style={s.input}
            value={unit}
            onChangeText={setUnit}
            placeholder="e.g. m², No., item"
            autoCapitalize="none"
          />
        </Field>

        <Field label="Quantity">
          <TextInput
            style={s.input}
            value={quantity}
            onChangeText={setQuantity}
            placeholder="e.g. 50 or 12.5"
            keyboardType="decimal-pad"
          />
        </Field>

        <Field label="Rate" hint="₦ per unit (e.g. 1234.50)">
          <TextInput
            style={s.input}
            value={rate}
            onChangeText={setRate}
            placeholder="0.00"
            keyboardType="decimal-pad"
          />
        </Field>

        <Field label="Type">
          <View style={s.segmented}>
            {ITEM_TYPES.map((opt) => {
              const selected = opt.value === itemType;
              return (
                <Pressable
                  key={opt.value}
                  style={[s.segment, selected && s.segmentSelected]}
                  onPress={() => setItemType(opt.value)}
                >
                  <Text
                    style={[
                      s.segmentText,
                      selected && s.segmentTextSelected,
                    ]}
                  >
                    {opt.label}
                  </Text>
                </Pressable>
              );
            })}
          </View>
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
            <Text style={s.buttonText}>Create item</Text>
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
  segmented: {
    flexDirection: 'row',
    borderWidth: 1,
    borderColor: '#ccc',
    borderRadius: 10,
    overflow: 'hidden',
  },
  segment: {
    flex: 1,
    paddingVertical: 10,
    alignItems: 'center',
    backgroundColor: 'white',
  },
  segmentSelected: { backgroundColor: '#208AEF' },
  segmentText: { fontSize: 13, color: '#333' },
  segmentTextSelected: { color: 'white', fontWeight: '600' },
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
