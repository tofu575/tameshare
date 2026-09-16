import "server-only";

import { getBackendApiUrl } from "./getBackendApiUrl";

// Generated Clientの相対URLをBackend URLへ接続してFetchする。
export async function apiFetch<T>(
  url: string,
  options: RequestInit,
): Promise<T> {
  const response = await fetch(`${getBackendApiUrl()}${url}`, options);
  const body = [204, 205, 304].includes(response.status)
    ? null
    : await response.text();
  const data = body ? JSON.parse(body) : {};
  return {
    data,
    status: response.status,
    headers: response.headers,
  } as T;
}
