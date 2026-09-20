// BackendのHTTP statusと安定したerror codeを保持する。
export class ApiResponseError extends Error {
  constructor(
    readonly status: number,
    readonly code: string,
    message: string,
  ) {
    super(message);
    this.name = "ApiResponseError";
  }
}
