import { Stack } from 'expo-router';

export default function ProjectLayout() {
  return (
    <Stack>
      <Stack.Screen name="index" />
      <Stack.Screen
        name="new-section"
        options={{ presentation: 'modal', title: 'New section' }}
      />
      <Stack.Screen
        name="sections/[sectionId]/new-item"
        options={{ presentation: 'modal', title: 'New item' }}
      />
    </Stack>
  );
}
