import { ref } from "vue"
import { invoke } from "@tauri-apps/api"
import { AppError } from "./errortype"

export type LoginBigSeller = {
    email: string
    password: string
    captcha: string
    cookie_string: string
}

export const loginBigSeller = (options?: {
    onSuccess?: (msg: string) => void
    onError?: (e: AppError) => void
}) => {
    const pending = ref(false)
    const errorString = ref<null | string>(null)
    const data = ref<null | string>(null)

    const invoker = async (payload: LoginBigSeller) => {
        pending.value = true

        try {
            const response = await invoke<string>("login_bigseller", {
                payload: {
                    ...payload,
                },
            })

            errorString.value = null
            data.value = response

            options?.onSuccess?.(response)
        } catch (e) {
            errorString.value = JSON.stringify(e)
            data.value = null

            options?.onError?.(e as AppError)
        } finally {
            pending.value = false
        }
    }

    return {
        pending,
        errorString,
        data,
        invoker,
    }
}
