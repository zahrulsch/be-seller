import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { ShopeeConfigData } from "../types/ShopeeConfigData";

export const createConfigShopee = (options?: {
  onSuccess?: (msg: string) => void;
  onError?: (e: any) => void;
}) => {
  const pending = ref(false);
  const errorString = ref<string | null>(null);

  const invoker = async (data: ShopeeConfigData) => {
    pending.value = true;
    errorString.value = null;

    try {
      const response = await invoke<string>("add_config_shopee", {
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
