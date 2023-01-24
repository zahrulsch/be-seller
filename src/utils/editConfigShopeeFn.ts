import { invoke } from '@tauri-apps/api';
import { ShopeeConfigData } from './../types/ShopeeConfigData';
import { ref } from "vue";

export const editConfigShopee = (options?: {
  onSuccess?: (message: string) => void;
  onError?: (e: any) => void
}) => {
  const pending = ref(false);
  const errorString = ref<string|null>(null);

  const invoker = (data: ShopeeConfigData) => {
    pending.value = true
    invoke<string>("edit_config_shopee", {
      data,
    })
      .then(msg => {
        options?.onSuccess?.(msg)
      })
      .catch(e => {
        errorString.value = String(e);
        options?.onError?.(e)
      })
      .finally(() => {
        pending.value = false;
      });
  }

  return {
    invoker,
    pending
  }

}