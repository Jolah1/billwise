import {
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query';

import * as api from '@/api/projects';
import type { CreateProjectReq } from '@/api/types';
import { useAuthToken } from '@/auth/context';

export const projectsKey = ['projects'] as const;
export const projectDetailKey = (id: string) =>
  ['projects', id, 'detail'] as const;

export function useProjects() {
  const { token } = useAuthToken();
  return useQuery({
    queryKey: projectsKey,
    queryFn: () => api.listProjects(token),
  });
}

export function useProjectDetail(projectId: string) {
  const { token } = useAuthToken();
  return useQuery({
    queryKey: projectDetailKey(projectId),
    queryFn: () => api.getProjectDetail(token, projectId),
  });
}

export function useCreateProject() {
  const { token } = useAuthToken();
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (body: CreateProjectReq) => api.createProject(token, body),
    onSuccess: () => qc.invalidateQueries({ queryKey: projectsKey }),
  });
}
