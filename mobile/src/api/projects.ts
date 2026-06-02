import { apiFetch } from './client';
import type {
  CreateProjectReq,
  ProjectDetailResp,
  ProjectResp,
} from './types';

export const listProjects = (token: string) =>
  apiFetch<ProjectResp[]>('/projects', { token });

export const createProject = (token: string, body: CreateProjectReq) =>
  apiFetch<ProjectResp>('/projects', {
    method: 'POST',
    token,
    body: JSON.stringify(body),
  });

export const getProjectDetail = (token: string, projectId: string) =>
  apiFetch<ProjectDetailResp>(`/projects/${projectId}`, { token });
