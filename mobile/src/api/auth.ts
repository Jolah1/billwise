import { apiFetch } from './client';
import type { AuthResp, LoginReq, RegisterReq } from './types';

export const register = (body: RegisterReq) =>
  apiFetch<AuthResp>('/auth/register', {
    method: 'POST',
    body: JSON.stringify(body),
  });

export const login = (body: LoginReq) =>
  apiFetch<AuthResp>('/auth/login', {
    method: 'POST',
    body: JSON.stringify(body),
  });
