<script setup lang="ts">
import { Modal, Space } from "ant-design-vue";

const props = defineProps<{
	show?: boolean;
}>();

const emits = defineEmits<{
	(e: "update:show", v: boolean): void;
}>();
</script>

<template>
	<Modal
		:visible="props.show"
		@update:visible="(v) => emits('update:show', v)"
		title="List Akun Big Seller"
		style="width: 580px"
		centered
	>
		<Space></Space>
	</Modal>
</template>

<style lang="scss"></style>
