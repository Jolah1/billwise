import { useMutation, useQueryClient } from '@tanstack/react-query';

import * as api from '@/api/items';
import type { CreateItemReq } from '@/api/types';
import { useAuthToken } from '@/auth/context';

import { projectDetailKey } from './projects';

export function useCreateItem(projectId: string, sectionId: string) {
  const { token } = useAuthToken();
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: CreateItemReq) =>
      api.createItem(token, sectionId, body),
    onSuccess: () =>
      qc.invalidateQueries({ queryKey: projectDetailKey(projectId) }),
  });
}
