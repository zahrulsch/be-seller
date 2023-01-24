import { invoke } from "@tauri-apps/api";
import { ShopeeConfigData } from "../types/ShopeeConfigData";
import { onMounted, ref } from "vue";

export const useGetConfig = () => {
  const pending = ref(false);
  const config = ref<ShopeeConfigData[]|null>(null);
  const errorString = ref<string|null>(null)

  const invoker = async () => {
    pending.value = true;
    errorString.value = null;

    try {
      const response = await invoke<ShopeeConfigData[]>("get_config_shopee");
      config.value = response
    } catch (e) {
      errorString.value = JSON.stringify(e);
    } finally {
      pending.value = false;
    }
    
  };

  onMounted(async () => {
    await invoker();
  });

  return {
    invoker,
    pending,
    config,
    errorString
  }
};
