/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

interface ImportMetaEnv {
  readonly VITE_ENABLE_AUTH?: string;
  readonly VITE_LOGIN_ENDPOINT?: string;
  readonly VITE_ENCRYPTION_KEY?: string;
  readonly VITE_GET_USER_INFO_ENDPOINT?: string;
  readonly VITE_GET_MEMBERSHIP_GSJ_ENDPOINT?: string;
  readonly VITE_ACTIVATE_MEMBERSHIP_GSJ_ENDPOINT?: string;
  readonly VITE_GET_DASHSCOPE_TOKEN_GSJ_ENDPOINT?: string;
  readonly VITE_TEMPLATE_ENCRYPTION_KEY?: string;
  readonly VITE_TEMP_API_ENCRYPTION_KEY?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
