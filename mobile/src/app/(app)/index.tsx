import {
  ActivityIndicator,
  FlatList,
  Pressable,
  RefreshControl,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { Link, useRouter } from 'expo-router';

import { ApiError } from '@/api/client';
import type { ProjectResp } from '@/api/types';
import { useAuth } from '@/auth/context';
import { useProjects } from '@/queries/projects';

export default function ProjectsList() {
  const router = useRouter();
  const { signOut } = useAuth();
  const { data, isLoading, error, refetch, isRefetching } = useProjects();

  if (isLoading) {
    return (
      <View style={s.centered}>
        <ActivityIndicator />
      </View>
    );
  }

  if (error) {
    const msg =
      error instanceof ApiError ? error.message : 'Could not load projects';
    return (
      <View style={s.centered}>
        <Text style={s.error}>{msg}</Text>
        <Pressable style={s.retryButton} onPress={() => refetch()}>
          <Text style={s.retryText}>Try again</Text>
        </Pressable>
      </View>
    );
  }

  const projects = data ?? [];

  return (
    <View style={s.fill}>
      <FlatList
        data={projects}
        keyExtractor={(p) => p.id}
        renderItem={({ item }) => <ProjectRow project={item} />}
        ItemSeparatorComponent={() => <View style={s.separator} />}
        contentContainerStyle={projects.length === 0 ? s.emptyContent : undefined}
        ListEmptyComponent={
          <View style={s.empty}>
            <Text style={s.emptyTitle}>No projects yet</Text>
            <Text style={s.emptySubtitle}>
              Tap “New project” to create your first BOQ.
            </Text>
          </View>
        }
        refreshControl={
          <RefreshControl refreshing={isRefetching} onRefresh={refetch} />
        }
      />

      <View style={s.footer}>
        <Pressable
          style={s.primaryButton}
          onPress={() => router.push('/(app)/new-project')}
        >
          <Text style={s.primaryButtonText}>New project</Text>
        </Pressable>
        <Pressable style={s.secondaryButton} onPress={signOut}>
          <Text style={s.secondaryButtonText}>Sign out</Text>
        </Pressable>
      </View>
    </View>
  );
}

function ProjectRow({ project }: { project: ProjectResp }) {
  return (
    <Link href={`/(app)/projects/${project.id}`} asChild>
      <Pressable style={({ pressed }) => [s.row, pressed && s.rowPressed]}>
        <Text style={s.rowTitle}>{project.title}</Text>
        <Text style={s.rowMeta}>
          {project.client} · {project.location}
        </Text>
        <Text style={s.rowDate}>{project.project_date}</Text>
      </Pressable>
    </Link>
  );
}

const s = StyleSheet.create({
  fill: { flex: 1, backgroundColor: 'white' },
  centered: { flex: 1, justifyContent: 'center', alignItems: 'center', padding: 24 },
  row: { paddingVertical: 16, paddingHorizontal: 20, backgroundColor: 'white' },
  rowPressed: { backgroundColor: '#f4f6fb' },
  rowTitle: { fontSize: 16, fontWeight: '600' },
  rowMeta: { fontSize: 13, color: '#666', marginTop: 4 },
  rowDate: { fontSize: 12, color: '#999', marginTop: 4 },
  separator: { height: StyleSheet.hairlineWidth, backgroundColor: '#e5e5e5' },
  emptyContent: { flex: 1, justifyContent: 'center' },
  empty: { padding: 32, alignItems: 'center', gap: 6 },
  emptyTitle: { fontSize: 18, fontWeight: '600' },
  emptySubtitle: { color: '#666', textAlign: 'center' },
  footer: {
    padding: 16,
    gap: 8,
    borderTopWidth: StyleSheet.hairlineWidth,
    borderTopColor: '#e5e5e5',
    backgroundColor: 'white',
  },
  primaryButton: {
    backgroundColor: '#208AEF',
    paddingVertical: 14,
    borderRadius: 10,
    alignItems: 'center',
  },
  primaryButtonText: { color: 'white', fontWeight: '600', fontSize: 16 },
  secondaryButton: { paddingVertical: 10, alignItems: 'center' },
  secondaryButtonText: { color: '#666', fontSize: 14 },
  error: { color: '#c0392b', marginBottom: 12 },
  retryButton: { paddingHorizontal: 20, paddingVertical: 10, backgroundColor: '#208AEF', borderRadius: 8 },
  retryText: { color: 'white', fontWeight: '600' },
});
