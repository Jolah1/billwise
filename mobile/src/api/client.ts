import { API_BASE_URL } from './config';
import type { ApiErrorBody } from './types';

// Thrown on any non-2xx response. The `code` mirrors AppError::code()
// on the server ("unauthorized", "not_found", "bad_request", ...) so
// callers can branch on it instead of parsing the message.
export class ApiError extends Error {
  readonly status: number;
  readonly code: string;
  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.code = code;
  }
}

export interface ApiFetchOptions extends RequestInit {
  token?: string;
}

export async function apiFetch<T>(
  path: string,
  { token, headers, ...init }: ApiFetchOptions = {},
): Promise<T> {
  const h = new Headers(headers);
  if (!h.has('content-type')) h.set('content-type', 'application/json');
  if (!h.has('accept')) h.set('accept', 'application/json');
  if (token) h.set('authorization', `Bearer ${token}`);

  const res = await fetch(`${API_BASE_URL}${path}`, { ...init, headers: h });

  if (!res.ok) {
    let body: ApiErrorBody;
    try {
      body = (await res.json()) as ApiErrorBody;
    } catch {
      body = { error: 'unknown', message: `HTTP ${res.status}` };
    }
    throw new ApiError(res.status, body.error, body.message);
  }

  if (res.status === 204) return undefined as T;
  return (await res.json()) as T;
}
