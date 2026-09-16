import "server-only";

// 必須のBackend API URLを末尾スラッシュなしで返す。
export function getBackendApiUrl(): string {
  const backendApiUrl = process.env.BACKEND_API_URL;
  if (!backendApiUrl) {
    throw new Error("BACKEND_API_URL is required");
  }
  return backendApiUrl.replace(/\/$/, "");
}
