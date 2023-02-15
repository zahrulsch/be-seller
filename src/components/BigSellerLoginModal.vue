<script setup lang="ts">
import {
	Modal,
	Space,
	Input,
	Image,
	Divider,
	TypographyText,
	InputPassword,
	Spin,
	notification,
	message,
} from "ant-design-vue";
import { ref, watch } from "vue";
import { getBigSellerPreLoginData } from "../utils/getBigSellerPreLoginData";
import { loginBigSeller, LoginBigSeller } from "../utils/loginBigSeller";

const image = ref("");
const { invoker, pending } = getBigSellerPreLoginData({
	onSuccess: (data) => {
		image.value = data.v_code_response.data;
		payload.value.cookie_string = data.cookies;
	},
});

const { invoker: loginProcess } = loginBigSeller({
	onError: (e) => {
		console.log(e);
		invoker();
		notification.error({
			message: e.name,
			description: e.cause,
			placement: "bottomRight",
		});
	},
	onSuccess: () => {
		payload.value = {
			captcha: "",
			cookie_string: "",
			email: "",
			password: "",
		};
		invoker();
		message.success("Login berhasil silahkan lanjutkan proses", 2.4);
	},
});

interface Prop {
	show?: boolean;
}

interface Emit {
	(e: "update:show", v: boolean): void;
}

const props = defineProps<Prop>();
const emits = defineEmits<Emit>();

const payload = ref<LoginBigSeller>({
	captcha: "",
	cookie_string: "",
	email: "",
	password: "",
});

watch(
	() => props.show,
	(s) => {
		if (s) {
			invoker();
		} else {
			payload.value = {
				captcha: "",
				cookie_string: "",
				email: "",
				password: "",
			};
		}
	}
);
</script>

<template>
	<Modal
		:visible="props.show"
		@update:visible="(v) => emits('update:show', v)"
		centered
		title="Tambahkan Akun Big Seller"
		style="width: 380px"
		ok-text="Simpan"
		cancel-text="Kembali"
		@ok="
			() => {
				loginProcess({ ...payload });
			}
		"
		:mask-style="{
			backgroundColor: 'rgba(0, 0, 0, .08)',
		}"
	>
		<Space style="width: 100%" direction="vertical">
			<Input
				v-model:value="payload.email"
				addon-before="Email"
				placeholder="son_goku@yahoo.com"
			/>
			<InputPassword
				v-model:value="payload.password"
				placeholder="••••••••"
				addon-before="Password"
			/>
			<Divider style="margin-block: 4px">
				<TypographyText style="font-size: 13px; font-weight: 400">
					Masukan Captcha
				</TypographyText>
			</Divider>
			<div v-if="pending" class="spinner">
				<Spin size="small" />
			</div>
			<Image
				v-else
				:src="image"
				width="100%"
				height="auto"
				style="border-radius: 2px"
				:preview="false"
			/>
			<Input v-model:value="payload.captcha" addon-before="Captcha"> </Input>
		</Space>
	</Modal>
</template>

<style lang="scss">
.spinner {
	display: flex;
	width: 100%;
	min-height: 100px;
	justify-content: center;
	align-items: center;
}
</style>
