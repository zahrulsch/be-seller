import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

export type ColsData = ColData[];

export interface ColData {
    config_id: number;
    created_at: string;
    id: number;
    max_price_product: number;
    min_price_product: number;
    name: string;
    product_count: number;
}

export const getColDb = (options?: {
    onSuccess?: (msg: ColsData) => void;
    onError?: (e: any) => void;
}) => {
    const pending = ref(false);
    const errorString = ref<null | string>(null);
    const data = ref<null | ColsData>(null);

    const invoker = async () => {
        pending.value = true;

        try {
            const response = await invoke<ColsData>("get_col_db");

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
