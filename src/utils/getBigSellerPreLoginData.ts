import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

interface BigSellerPreLoginData {
    cookies: Record<string, string>
    v_code_response: {
        code: number,
        data: string,
        msg: any
    }
}

export const getBigSellerPreLoginData = (options?: {
    onSuccess?: (msg: BigSellerPreLoginData) => void,
    onError?: (e: any) => void
}) => {
    const pending = ref(false);
    const errorString = ref<null | string>(null);
    const data = ref<null | BigSellerPreLoginData>(null);

    const invoker = async () => {
        pending.value = true;

        try {
            const response = await invoke<BigSellerPreLoginData>("get_data_login_bigseller");

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
}