import { defineConfig } from "orval";

// Backend OpenAPI契約からFetchベースのAPI ClientとModelを生成する。
export default defineConfig({
  tameshare: {
    input: "../backend/docs/openapi/tameshare.openapi.yaml",
    output: {
      target: "./lib/generated/api",
      schemas: "./lib/generated/model",
      client: "fetch",
      mode: "tags-split",
      clean: true,
    },
  },
});
