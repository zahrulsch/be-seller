import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

export const removeConfigShopee = (options?: {
  onSuccess?: (msg: string) => void;
  onError?: (e: any) => void;
}) => {
  const pending = ref(false);
  const errorString = ref<string | null>(null);

  const invoker = async (data: { id: number }) => {
    pending.value = true;
    errorString.value = null;

    try {
      const response = await invoke<string>("remove_config_shopee", {
        data,
      });
      options?.onSuccess?.(response);
    } catch (e) {
      options?.onError?.(e);
      errorString.value = JSON.stringify(e);
    } finally {
      pending.value = false;
    }
  };

  return {
    pending,
    invoker,
    errorString,
  };
};
