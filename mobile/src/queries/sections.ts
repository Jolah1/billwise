import { useMutation, useQueryClient } from '@tanstack/react-query';

import * as api from '@/api/sections';
import type { CreateSectionReq } from '@/api/types';
import { useAuthToken } from '@/auth/context';

import { projectDetailKey } from './projects';

export function useCreateSection(projectId: string) {
  const { token } = useAuthToken();
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: CreateSectionReq) =>
      api.createSection(token, projectId, body),
    onSuccess: () =>
      qc.invalidateQueries({ queryKey: projectDetailKey(projectId) }),
  });
}
