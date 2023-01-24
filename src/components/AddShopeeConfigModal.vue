<script lang="ts" setup>
import { Modal, Input } from "ant-design-vue";
import { ref } from "vue";

interface Prop {
	show?: boolean;
}

interface Emit {
	(e: "update:show", value: boolean): void;
	(e: "ok", value: string): void;
}

const props = defineProps<Prop>();
const emits = defineEmits<Emit>();

const name = ref<string>("");
</script>

<template>
	<Modal
		centered
		title="Buat Config"
		:visible="props.show"
		@update:visible="(v) => emits('update:show', v)"
		cancel-text="Batal"
		:mask-style="{
			backgroundColor: '#e5e5e573',
		}"
		transition-name=""
		@ok="
			() => {
				emits('ok', name);
				emits('update:show', false);
				name = '';
			}
		"
	>
		<Input
			@press-enter="
				() => {
					emits('ok', name);
					emits('update:show', false);
					name = '';
				}
			"
			v-model:value="name"
			addon-before="Nama"
			placeholder="Config Terbaru"
		/>
	</Modal>
</template>
