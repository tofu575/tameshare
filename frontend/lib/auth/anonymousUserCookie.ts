// BackendのBearer UUIDを保持するHTTP-only Cookie設定。
export const anonymousUserCookie = {
  name: "tameshare_anonymous_user_id",
  options: {
    httpOnly: true,
    sameSite: "lax" as const,
    secure: process.env.NODE_ENV === "production",
    path: "/",
    maxAge: 60 * 60 * 24 * 365,
  },
};
