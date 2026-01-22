/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

interface ImportMetaEnv {
  readonly VITE_ENABLE_AUTH?: string
  readonly VITE_EMAS_APP_ID?: string
  readonly VITE_EMAS_SPACE_ID?: string
  readonly VITE_EMAS_CLIENT_SECRET?: string
  readonly VITE_EMAS_ENDPOINT?: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
