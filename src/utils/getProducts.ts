import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

export type Products = {
    page_count: number;
    products: Product[];
};

interface Product {
    collection_id: number;
    collection_name: string;
    item_id: number;
    name: string;
    price_max: number;
    price_min: number;
    shop_id: number;
    sold: number;
    thumbnail: string;
    url: string;
    view: number;
}

export const getProducts = (options?: {
    onSuccess?: (msg: Products) => void;
    onError?: (e: any) => void;
}) => {
    const pending = ref(false);
    const errorString = ref<null | string>(null);
    const data = ref<null | Products>(null);

    const invoker = async (parameters?: {
        col_id?: number;
        name?: string;
        page?: number;
        page_size?: number;
        order_by?: "pr_desc" | "pr_asc" | "sold_desc" | "sold_asc";
    }) => {
        pending.value = true;

        try {
            const response = await invoke<Products>("get_product_list", {
                parameters,
            });

            errorString.value = null;
            data.value = response;

            options?.onSuccess?.(response);
        } catch (e) {
            errorString.value = JSON.stringify(e);
            data.value = null;

            options?.onError?.(e);
        } finally {
            pending.value = false;
        }
    };

    return {
        pending,
        errorString,
        data,
        invoker,
    };
};
