import {
  ActivityIndicator,
  Pressable,
  RefreshControl,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { Stack, useLocalSearchParams, useRouter } from 'expo-router';

import { ApiError } from '@/api/client';
import type {
  ItemInDetailResp,
  ProjectDetailResp,
  SectionWithItemsResp,
} from '@/api/types';
import { formatNaira } from '@/money/format';
import { useProjectDetail } from '@/queries/projects';

export default function ProjectDetailScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const router = useRouter();
  const { data, isLoading, error, refetch, isRefetching } =
    useProjectDetail(id);

  if (isLoading) {
    return (
      <View style={s.centered}>
        <ActivityIndicator />
      </View>
    );
  }

  if (error) {
    const msg =
      error instanceof ApiError ? error.message : 'Could not load project';
    return (
      <View style={s.centered}>
        <Text style={s.errorText}>{msg}</Text>
      </View>
    );
  }

  if (!data) return null;

  return (
    <>
      <Stack.Screen options={{ title: data.title }} />
      <ScrollView
        style={s.fill}
        contentContainerStyle={s.scroll}
        refreshControl={
          <RefreshControl refreshing={isRefetching} onRefresh={refetch} />
        }
      >
        <Header project={data} />
        <TotalCard total={data.total_kobo} />

        {data.sections.length === 0 ? (
          <Text style={s.emptyText}>No sections yet.</Text>
        ) : (
          data.sections.map((section) => (
            <Section
              key={section.id}
              projectId={data.id}
              section={section}
            />
          ))
        )}

        <Pressable
          style={({ pressed }) => [
            s.addSectionButton,
            pressed && s.addSectionButtonPressed,
          ]}
          onPress={() => router.push(`/(app)/projects/${data.id}/new-section`)}
        >
          <Text style={s.addSectionText}>+ Add section</Text>
        </Pressable>
      </ScrollView>
    </>
  );
}

function Header({ project }: { project: ProjectDetailResp }) {
  return (
    <View style={s.header}>
      <Text style={s.subtle}>
        {project.client} · {project.location}
      </Text>
      <Text style={s.subtle}>{project.project_date}</Text>
    </View>
  );
}

function TotalCard({ total }: { total: number }) {
  return (
    <View style={s.totalCard}>
      <Text style={s.totalLabel}>Project total</Text>
      <Text style={s.totalAmount}>{formatNaira(total)}</Text>
    </View>
  );
}

function Section({
  projectId,
  section,
}: {
  projectId: string;
  section: SectionWithItemsResp;
}) {
  const router = useRouter();
  return (
    <View style={s.section}>
      <View style={s.sectionHeader}>
        <Text style={s.sectionName}>{section.name}</Text>
        <Text style={s.sectionSubtotal}>
          {formatNaira(section.subtotal_kobo)}
        </Text>
      </View>
      {section.items.length === 0 ? (
        <Text style={s.emptyItem}>No items</Text>
      ) : (
        section.items.map((item) => <Item key={item.id} item={item} />)
      )}
      <Pressable
        style={({ pressed }) => [s.addItemRow, pressed && s.addItemRowPressed]}
        onPress={() =>
          router.push(
            `/(app)/projects/${projectId}/sections/${section.id}/new-item`,
          )
        }
      >
        <Text style={s.addItemText}>+ Add item</Text>
      </Pressable>
    </View>
  );
}

function Item({ item }: { item: ItemInDetailResp }) {
  return (
    <View style={s.item}>
      <Text style={s.itemDescription}>{item.description}</Text>
      <View style={s.itemBreakdown}>
        <Text style={s.itemMeta}>
          {item.quantity} {item.unit} × {formatNaira(item.rate_kobo)}
        </Text>
        <Text style={s.itemAmount}>{formatNaira(item.amount_kobo)}</Text>
      </View>
    </View>
  );
}

const s = StyleSheet.create({
  fill: { flex: 1, backgroundColor: '#f6f7fb' },
  scroll: { padding: 16, gap: 12, paddingBottom: 32 },
  centered: { flex: 1, justifyContent: 'center', alignItems: 'center', padding: 24 },
  errorText: { color: '#c0392b' },

  header: { gap: 4 },
  subtle: { color: '#555', fontSize: 13 },

  totalCard: {
    backgroundColor: 'white',
    padding: 16,
    borderRadius: 12,
    gap: 4,
    marginVertical: 8,
  },
  totalLabel: { fontSize: 13, color: '#666' },
  totalAmount: { fontSize: 28, fontWeight: '700' },

  section: {
    backgroundColor: 'white',
    borderRadius: 12,
    paddingVertical: 4,
    overflow: 'hidden',
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'baseline',
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderBottomColor: '#e5e5e5',
  },
  sectionName: { fontSize: 16, fontWeight: '600' },
  sectionSubtotal: { fontSize: 14, fontWeight: '500', color: '#333' },

  item: {
    paddingHorizontal: 16,
    paddingVertical: 12,
    borderBottomWidth: StyleSheet.hairlineWidth,
    borderBottomColor: '#eee',
    gap: 4,
  },
  itemDescription: { fontSize: 15 },
  itemBreakdown: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  itemMeta: { fontSize: 12, color: '#666' },
  itemAmount: { fontSize: 14, fontWeight: '500' },

  emptyText: { color: '#666', textAlign: 'center', marginTop: 24 },
  emptyItem: { color: '#999', padding: 16 },

  addSectionButton: {
    marginTop: 8,
    paddingVertical: 14,
    borderRadius: 10,
    borderWidth: 1,
    borderColor: '#208AEF',
    alignItems: 'center',
    backgroundColor: 'white',
  },
  addSectionButtonPressed: { backgroundColor: '#eef5ff' },
  addSectionText: { color: '#208AEF', fontWeight: '600', fontSize: 15 },

  addItemRow: {
    paddingHorizontal: 16,
    paddingVertical: 12,
    alignItems: 'flex-start',
  },
  addItemRowPressed: { backgroundColor: '#f4f6fb' },
  addItemText: { color: '#208AEF', fontWeight: '500', fontSize: 13 },
});
