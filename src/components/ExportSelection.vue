<script setup lang="ts">
import { watch } from "vue";
import {
	Modal,
	Collapse,
	CollapsePanel,
	Skeleton,
	Space,
	Avatar,
	TypographyText,
	Checkbox,
} from "ant-design-vue";
import { getProfiles } from "../utils/getProfiles";
import { CaretRight } from "@vicons/carbon";
import bs from "../assets/bigseller.png";
import _ from "lodash";

const props = defineProps<{
	show?: boolean;
}>();

const emits = defineEmits<{
	(e: "update:show", v: boolean): void;
}>();

const { data, invoker, pending } = getProfiles();

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
		style="width: 460px"
		:visible="props.show"
		@update:visible="(b) => emits('update:show', b)"
		:mask-style="{
			backgroundColor: 'rgba(0, 0, 0, .14)',
		}"
		centered
		title="Akun & Sales Channel"
		:footer="false"
		:body-style="{
			maxHeight: '300px',
			overflowY: 'auto',
		}"
	>
		<Skeleton v-if="pending" />
		<Collapse v-else>
			<template #expandIcon="{ isActive }">
				<CaretRight
					:style="isActive ? 'transform: rotate(90deg)' : ''"
					style="width: 15px; transition: 190ms ease"
				/>
			</template>
			<CollapsePanel v-for="(profile, i) in data" :key="i">
				<template #header>
					<Space style="width: 100%">
						<Avatar size="small" :src="bs" />
						<TypographyText strong>{{ profile.account }}</TypographyText>
					</Space>
				</template>
				<Space direction="vertical">
					<Checkbox
						v-for="channel in profile.sales_channels"
						:key="channel.id"
						style="user-select: none"
						>{{ channel.platform_shop_name }} (
						{{ _.capitalize(channel.platform) }} )</Checkbox
					>
				</Space>
			</CollapsePanel>
		</Collapse>
	</Modal>
</template>
