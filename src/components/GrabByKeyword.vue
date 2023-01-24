<script setup lang="ts">
import {
	Modal,
	Textarea,
	InputNumber,
	Input,
	Space,
	Select,
	Alert,
	SelectOption,
} from "ant-design-vue";
import { useGetConfig } from "../utils/useGetConfig";

interface Prop {
	show?: boolean;
}

interface Emit {
	(e: "update:show", v: boolean): void;
}

const props = defineProps<Prop>();
const emits = defineEmits<Emit>();

const { config } = useGetConfig();
</script>

<template>
	<Modal
		centered
		title="Kumpulkan produk berdasarkan kata kunci"
		:visible="props.show"
		@update:visible="(v) => emits('update:show', v)"
		:mask-style="{
			backgroundColor: '#e5e5e560',
		}"
		ok-text="Jalankan Tugas"
		cancel-text="Batal"
		transition-name=""
	>
		<Space style="width: 100%" direction="vertical">
			<Alert show-icon message="Hanya mendukung marketplace Shopee"></Alert>
			<Textarea
				:auto-size="{
					minRows: 14,
					maxRows: 14,
				}"
				placeholder="Daftar kata kunci pisahkan dengan baris baru"
			></Textarea>
			<Input addon-before="Nama" placeholder="Keyword fashion" />
			<Space>
				<InputNumber addon-before="Thread Count" />
				<InputNumber addon-before="Batas Produk" />
			</Space>
			<Select placeholder="Filter Namespace" style="width: 100%">
				<SelectOption v-for="cfg in config" :value="cfg.id">{{
					cfg.name
				}}</SelectOption>
			</Select>
		</Space>
	</Modal>
</template>
