import { invoke } from "@tauri-apps/api";
import { ref, onMounted } from "vue";
import { ShopeeFilterConfig } from "../types/ShopeeFilterConfig";

interface Config {
  value: string | number;
  label: string;
}

export const useGetFilterConfigShopee = () => {
  const sellerTypes = ref<Config[]>();
  const shippingAgents = ref<Config[]>();
  const addresses = ref<Config[]>();
  const pending = ref(false);
  const errorString = ref<string | null>(null);
  const invoker = () => {
    pending.value = true;
    errorString.value = null;

    invoke<ShopeeFilterConfig>("get_filter_config")
      .then((data) => {
        const filterField = data.data.filter_configuration.filter_groups;

        const [shopTypesArray] = filterField.filter(
          (s) => s.name === "SHOP_TYPE"
        );
        if (shopTypesArray) {
          const namesToPick = shopTypesArray.filters.map((s) => s.name);
          const types = data.data.filter_configuration.filters
            .filter((f) => namesToPick.includes(f.name))
            .map((f) => ({
              value: f.type,
              label: f.translations[0]?.text || "",
            }));

          sellerTypes.value = types;
        }

        shippingAgents.value =
          data.data.filter_configuration.dynamic_filter_group_data.shippings.map(
            (s) => ({
              value: s.positionid,
              label: s.display_name,
            })
          );

        addresses.value =
          data.data.filter_configuration.dynamic_filter_group_data.locations.map(
            (l) => ({
              label: l.display_name,
              value: l.display_name,
            })
          );
      })
      .catch((e) => {
        console.log(e)
        errorString.value = JSON.stringify(e);
      })
      .finally(() => (pending.value = false));
  };

  onMounted(() => {
    invoker();
  });

  return {
    sellerTypes,
    shippingAgents,
    addresses,
    pending,
    errorString,
    invoker,
  };
};
