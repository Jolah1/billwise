import { apiFetch } from './client';
import type { CreateItemReq, ItemResp } from './types';

export const createItem = (
  token: string,
  sectionId: string,
  body: CreateItemReq,
) =>
  apiFetch<ItemResp>(`/sections/${sectionId}/items`, {
    method: 'POST',
    token,
    body: JSON.stringify(body),
  });
