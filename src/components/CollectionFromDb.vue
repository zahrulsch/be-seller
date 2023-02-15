<script lang="ts" setup>
import { ref, watch } from "vue";
import {
	Modal,
	Space,
	Table,
	TableColumnType,
	Skeleton,
	Tag,
	TypographyText,
	Button,
	Tooltip,
} from "ant-design-vue";
import { ColData, getColDb } from "../utils/getColDb";
import { Delete, DocumentExport } from "@vicons/carbon";
import dayjs from "dayjs";
import id from "dayjs/locale/id";

const props = defineProps<{
	show?: boolean;
}>();

const emits = defineEmits<{
	(e: "update:show", b: boolean): void;
}>();

const { invoker, data, pending } = getColDb({
	onSuccess: console.log,
	onError: console.log,
});

const priceChanger = (price: number) => {
	return price.toLocaleString("id-ID", {
		currency: "IDR",
		style: "currency",
		maximumFractionDigits: 0,
	});
};

const dateChange = (dateString: string) => {
	const date = dayjs(dateString).locale(id).format("DD MMMM YYYY (HH:mm)");
	return date;
};

const columns: TableColumnType<ColData>[] = [
	{
		title: "No.",
		key: "key",
		dataIndex: "key",
	},
	{
		title: "Nama Koleksi",
		key: "name",
		dataIndex: "name",
	},
	{
		title: "Produk",
		key: "product_count",
		dataIndex: "product_count",
	},
	{
		title: "Rentang Harga",
		key: "price_range",
	},
	{
		title: "Dibuat Pada",
		key: "created_at",
		dataIndex: "created_at",
	},
	{
		title: "Action",
		key: "action",
	},
];

watch(
	() => props.show,
	(show) => {
		if (show) {
			invoker();
		}
	}
);
</script>

<template>
	<Modal
		:visible="props.show"
		@update:visible="(e) => emits('update:show', e)"
		title="Daftar Koleksi Produk di Database"
		style="width: 860px; max-height: 600px; overflow-y: auto"
		:body-style="{ display: 'flex' }"
		centered
		:mask-style="{
			backgroundColor: 'rgba(0, 0, 0, .08)',
		}"
	>
		<template #footer>
			<Button @click="emits('update:show', !1)">Kembali</Button>
		</template>
		<Space style="width: 100%" direction="vertical">
			<Skeleton v-if="pending"></Skeleton>
			<Table
				v-else
				bordered
				:data-source="data ? data : undefined"
				:columns="columns"
				:pagination="false"
			>
				<template #bodyCell="{ column, index, record }">
					<template v-if="column.key === 'key'">
						<span>{{ index + 1 }}.</span>
					</template>
					<template v-else-if="column.key === 'price_range'">
						<Space size="small">
							<Tag
								style="margin: 0"
								>{{ priceChanger(record.min_price_product as number) }}</Tag
							>
							-
							<Tag
								style="margin: 0"
								>{{ priceChanger(record.max_price_product as number) }}</Tag
							>
						</Space>
					</template>
					<template v-else-if="column.key === 'created_at'">
						<TypographyText>
							{{ dateChange(record.created_at as string) }}
						</TypographyText>
					</template>
					<template v-else-if="column.key === 'action'">
						<Space size="small">
							<Tooltip placement="left">
								<template #title>
									<span>Hapus</span>
								</template>
								<Button size="small" type="default" danger>
									<template #icon>
										<Delete style="width: 11.7px" />
									</template>
								</Button>
							</Tooltip>
							<Tooltip placement="right">
								<RouterLink to="/bs-upload">
									<Button ghost size="small" type="primary">
										<template #icon>
											<DocumentExport style="width: 12px" />
										</template>
									</Button>
								</RouterLink>
								<template #title>
									<span>Upload dengan Big Seller</span>
								</template>
							</Tooltip>
						</Space>
					</template>
				</template>
			</Table>
		</Space>
	</Modal>
</template>
