import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

type AccountInfos = AccountInfo[];

interface AccountInfo {
    account: string;
    id: number;
    status: number;
    cookie_filename: string;
    sales_channels: SalesChannel[];
}

interface SalesChannel {
    auth_time: number;
    auth_time_str: string;
    id: number;
    name: string;
    parent_shop_id: any;
    platform: string;
    platform_shop_name: string;
    shop_status: string;
}

export const getProfiles = (options?: {
    onSuccess?: (msg: AccountInfos) => void;
    onError?: (e: any) => void;
}) => {
    const pending = ref(false);
    const errorString = ref<null | string>(null);
    const data = ref<null | AccountInfos>(null);

    const invoker = async () => {
        pending.value = true;

        try {
            const response = await invoke<AccountInfos>("get_profiles");

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
