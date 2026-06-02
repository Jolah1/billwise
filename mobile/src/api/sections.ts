import { apiFetch } from './client';
import type { CreateSectionReq, SectionResp } from './types';

export const listSections = (token: string, projectId: string) =>
  apiFetch<SectionResp[]>(`/projects/${projectId}/sections`, { token });

export const createSection = (
  token: string,
  projectId: string,
  body: CreateSectionReq,
) =>
  apiFetch<SectionResp>(`/projects/${projectId}/sections`, {
    method: 'POST',
    token,
    body: JSON.stringify(body),
  });
