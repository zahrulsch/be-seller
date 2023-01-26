import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

export type CrawlByKeyword = {
    keywords: string[];
    thread_size: number;
    name: string;
    limit_product: number;
    config_id: number;
};

export const crawlByKeyword = (options?: {
    onSuccess?: (msg: string) => void,
    onError?: (e: any) => void
}) => {
    const pending = ref(false);
    const errorString = ref<null | string>(null);
    const data = ref<null | string>(null);

    const invoker = async (payload: Partial<CrawlByKeyword>) => {
        pending.value = true;
        
        try {
            const response = await invoke<string>("crawl_by_keywords", {
                data: { ...payload },
            });

            errorString.value = null;
            data.value = response;

            options?.onSuccess?.(response);
        } catch (e) {
            errorString.value = JSON.stringify(e);
            data.value = null;

            options?.onError?.(e)
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
