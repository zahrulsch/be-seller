<script setup lang="ts">
import {
	Card,
	Space,
	Button,
	Divider,
	Select,
	SelectProps,
	TypographyText,
	Checkbox,
	Row,
	Col,
	CardMeta,
	Image,
} from "ant-design-vue";
import { useRouter } from "vue-router";
import { computed, onMounted, ref, watch } from "vue";
import { getProfiles } from "../utils/getProfiles";
import { getProducts } from "../utils/getProducts";
import ProductSearchFilter from "../components/ProductSearchFilter.vue";
import _ from "lodash";

const router = useRouter();
const selectedProfile = ref<SelectProps["value"]>();
const selectedChannels = ref([]);

const pageFilter = ref({
	name: "",
	col_id: undefined,
	page: 1,
	page_size: 20,
	order_by: "pr_desc" as "pr_desc" | "pr_asc" | "sold_desc" | "sold_asc",
});

const { data, invoker } = getProfiles();
const { invoker: productsGetter, data: products } = getProducts({
	onError: console.log,
	onSuccess: console.log,
});

const profileSelection = computed((): SelectProps["options"] => {
	if (data.value?.length) {
		return data.value.map((data, index) => {
			if (index === 0) {
				selectedProfile.value = data.id;
			}

			return {
				label: data.account,
				value: data.id,
			};
		});
	}

	return undefined;
});

const activeProfileChannels = computed(() => {
	if (selectedProfile.value && data.value) {
		return _.flatten(
			data.value.map((data) => {
				return data.sales_channels.map((sales) => {
					return {
						display_name: sales.platform_shop_name,
						shop_id: sales.id,
						shop_status: sales.shop_status,
						platform: sales.platform,
					};
				});
			})
		);
	}

	return [];
});

const priceChanger = (price: number) => {
	return price.toLocaleString("id-ID", {
		currency: "IDR",
		style: "currency",
		maximumFractionDigits: 0,
	});
};

const localeChanger = (price: number) => {
	return price.toLocaleString("id-ID", {
		maximumFractionDigits: 0,
	});
};

onMounted(() => {
	invoker();
	productsGetter({
		...pageFilter.value,
	});
});

watch(
	() => pageFilter.value,
	(npf) => {
		productsGetter({ ...npf });
	},
	{ deep: true }
);
</script>

<template>
	<div class="container">
		<div class="primer_container">
			<Card
				:body-style="{
					display: 'flex',
				}"
				title="Upload dengan Big Seller"
				style="width: 100%"
				size="small"
			>
				<div size="small" class="outer">
					<!-- space atas -->
					<div class="profile-selector">
						<Space style="width: 20%; padding: 8px 2px" direction="vertical">
							<TypographyText style="font-weight: 500; font-size: 13px"
								>Pilih Profile Big Seller</TypographyText
							>
							<Select
								style="width: 100%"
								placeholder="Pilih profile"
								:options="profileSelection"
								v-model:value="selectedProfile"
							></Select>
						</Space>
						<Divider style="height: auto" type="vertical" />
						<Space align="start" wrap>
							<Checkbox v-for="channel in activeProfileChannels"
								>{{ channel.display_name }} -
								{{ _.capitalize(channel.platform) }} ({{
									_.capitalize(channel.shop_status)
								}})</Checkbox
							>
						</Space>
					</div>
					<Divider style="margin-block: 5px"> </Divider>
					<div class="product-list-container">
						<TypographyText style="font-weight: 500; font-size: 13px"
							>Pilihan Produk</TypographyText
						>
						<ProductSearchFilter
							:page-size="pageFilter.page_size"
							:item-count="products?.page_count"
							v-model:current="pageFilter.page"
							:name="pageFilter.name"
							:collection-id="pageFilter.col_id"
							:order-by="pageFilter.order_by"
							@update:order-by="
								(b) => {
									pageFilter.page = 1;
									pageFilter.order_by = b;
								}
							"
							@update:collection-id="
								(v) => {
									pageFilter.page = 1;
									pageFilter.col_id = v;
								}
							"
							@update:name="
								(n) => {
									pageFilter.page = 1;
									pageFilter.name = n;
								}
							"
						/>
						<Row :gutter="[12, 12]">
							<Col
								v-for="p in products?.products"
								:key="p.item_id"
								:span="4"
							>
								<Card
									style="overflow: hidden"
									:bordered="!!1"
									hoverable
									size="small"
								>
									<Space
										style="width: 100%"
										size="middle"
										direction="vertical"
									>
										<Image
											:preview="!1"
											:alt="p.name"
											:src="p.thumbnail"
											crossorigin="anonymous"
										></Image>
										<card-meta>
											<template #description>
												<Space direction="vertical">
													<TypographyText strong type="warning"
														>{{ priceChanger(p.price_min) }} -
														{{
															priceChanger(p.price_max)
														}}</TypographyText
													>
													<span>
														Terjual :
														{{ localeChanger(p.sold) }}
													</span>
													<span>
														Koleksi : {{ p.collection_name }}
													</span>
												</Space>
											</template>
											<template #title>
												<span
													class="title"
													:strong="!1"
													style=""
													>{{ p.name }}</span
												>
											</template>
										</card-meta>
									</Space>
								</Card>
							</Col>
						</Row>
					</div>
					<Divider style="margin-block: 5px"> </Divider>
					<Space>
						<Button @click="router.back()">Kembali</Button>
						<Button type="primary">Buat Sesi</Button>
					</Space>
				</div>
			</Card>
		</div>
	</div>
</template>

<style lang="scss" scoped>
.outer {
	width: 100%;
	display: flex;
	flex-direction: column;
	row-gap: 8px;
}
.title {
	width: calc(100% - 10px);
	text-overflow: ellipsis;
	overflow: hidden;
	white-space: pre-line;
	-webkit-line-clamp: 2;
	-webkit-box-orient: vertical;
	display: -webkit-box;
	font-size: 13.6px;
}
.container {
	width: 100vw;
	display: flex;
	box-sizing: border-box;
	padding: 20px;
	justify-content: center;
	overflow-y: auto;
	overflow-x: hidden;

	.primer_container {
		box-sizing: border-box;
		width: 100%;
		max-width: 1500px;
		display: flex;
		column-gap: 7px;
		row-gap: 7px;
		flex-wrap: wrap;
		flex-direction: column;
		height: 100%;
	}
	.profile-selector {
		display: flex;
		align-items: stretch;
		justify-content: flex-start;
		column-gap: 5px;
	}
	.product-list-container {
		display: flex;
		flex-direction: column;
		row-gap: 8px;
	}
}
</style>
