<script lang="ts" setup>
import { Card, Pagination, Input, Select, SelectProps } from "ant-design-vue";
import { computed, onMounted } from "vue";
import { getColDb } from "../utils/getColDb";

type Nu = "pr_desc" | "pr_asc" | "sold_desc" | "sold_asc";

const props = defineProps<{
	pageSize?: number;
	itemCount?: number;
	current?: number;
	name?: string;
	collectionId?: number;
	orderBy?: Nu;
}>();

const emits = defineEmits<{
	(e: "update:current", n: number): void;
	(e: "update:name", n: string): void;
	(e: "update:collection-id", n: any): void;
	(e: "update:order-by", n: any): void;
}>();

const { data, invoker } = getColDb();

const cols = computed((): SelectProps["options"] => {
	if (data.value) {
		return data.value.map((data) => ({
			label: data.name,
			value: data.id,
		}));
	}
	return [];
});

const ordering = [
	{ value: "pr_desc", label: "Harga (Desc)" },
	{ value: "pr_asc", label: "Harga (Asc)" },
	{ value: "sold_desc", label: "Terjual (Desc)" },
	{ value: "sold_asc", label: "Terjual (Asc)" },
];

onMounted(invoker);
</script>

<template>
	<Card size="small">
		<div class="filter" style="width: 100%">
			<Input
				:value="props.name"
				@update:value="(v) => emits('update:name', v)"
				style="flex: 1"
				placeholder="Nama produk"
				allow-clear
			/>
			<Select
				:value="props.orderBy"
				@update:value="(e) => emits('update:order-by', e)"
				:options="ordering"
				placeholder="Urutkan"
				style="width: 200px"
			/>
			<Select
				:value="props.collectionId"
				allow-clear
				@update:value="(v) => emits('update:collection-id', v)"
				:options="cols"
				placeholder="Koleksi"
				style="width: 300px"
			/>
			<Pagination
				:page-size="props.pageSize"
				:total="props.itemCount"
				:size="'default'"
				:show-size-changer="false"
				:current="props.current"
				@update:current="(e) => emits('update:current', e)"
			/>
		</div>
	</Card>
</template>

<style lang="scss" scoped>
.filter {
	display: flex;
	column-gap: 7px;
}
</style>
