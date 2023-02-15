<script setup lang="ts">
import {
	Modal,
	Space,
	Skeleton,
	Collapse,
	CollapsePanel,
	Avatar,
	TypographyText,
	Table,
	Tag,
} from "ant-design-vue";
import { getProfiles } from "../utils/getProfiles";
import { watch, ref } from "vue";
import _ from "lodash";
import bs from "../assets/bigseller.png";

const columns = [
	{
		title: "Nickname",
		dataIndex: "name",
		key: "name",
		fixed: true,
	},
	{
		title: "Store Name",
		dataIndex: "platform_shop_name",
		key: "platform_shop_name",
	},
	{
		title: "Platform",
		dataIndex: "platform",
		key: "platform",
	},
	{
		title: "Status",
		dataIndex: "shop_status",
		key: "shop_status",
	},
	{
		title: "Tanggal Login",
		key: "auth_time_str",
		dataIndex: "auth_time_str",
	},
];

const props = defineProps<{
	show?: boolean;
}>();

const emits = defineEmits<{
	(e: "update:show", v: boolean): void;
}>();

const activeKey = ref(0);
const { invoker, data, pending } = getProfiles({
	onSuccess: console.log,
	onError: console.log,
});

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
		@update:visible="(v) => emits('update:show', v)"
		title="List Akun Big Seller"
		style="width: 620px"
		centered
	>
		<Space style="width: 100%" direction="vertical">
			<Skeleton v-if="pending" />
			<Collapse v-model:active-key="activeKey" v-else>
				<CollapsePanel v-for="(profile, i) in data" :key="i" :show-arrow="false">
					<template #header>
						<Space style="width: 100%">
							<Avatar size="small" :src="bs" />
							<TypographyText strong>{{ profile.account }}</TypographyText>
						</Space>
					</template>
					<Table
						bordered
						:columns="columns"
						:pagination="false"
						:data-source="profile.sales_channels"
						:scroll="{ x: 780 }"
					>
						<template #bodyCell="{ column, record }">
							<template v-if="column.key === 'shop_status'">
								<Tag
									:color="
										record.shop_status === 'Connected'
											? 'success'
											: ''
									"
									>{{ record.shop_status }}</Tag
								>
							</template>
							<template v-else-if="column.key === 'platform'">
								<Tag>
									{{ _.capitalize(record.platform) }}
								</Tag>
							</template>
						</template>
					</Table>
				</CollapsePanel>
			</Collapse>
		</Space>
	</Modal>
</template>

<style lang="scss"></style>
