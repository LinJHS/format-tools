/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

interface ImportMetaEnv {
  readonly VITE_ENABLE_AUTH?: string
  readonly VITE_LOGIN_ENDPOINT?: string
  readonly VITE_ENCRYPTION_KEY?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
