<script lang="ts" setup>
import {
	Card,
	Form,
	FormItem,
	Checkbox,
	InputGroup,
	InputNumber,
	Rate,
	Row,
	Col,
	Input,
	Space,
	Select,
	Textarea,
	Button,
	Divider,
	SelectOption,
	Alert,
	Tag,
	Spin,
	TypographyText,
	message,
} from "ant-design-vue";
import { ref, watch } from "vue";
import { useRouter } from "vue-router";
import { useGetFilterConfigShopee } from "../utils/useGetFIlterConfigShopee";
import { useGetConfig } from "../utils/useGetConfig";
import { ShopeeConfigData } from "../types/ShopeeConfigData";
import { editConfigShopee } from "../utils/editConfigShopeeFn";
import { createConfigShopee } from "../utils/createConfigShopeeFn";
import { removeConfigShopee } from "../utils/removeConfigShopeeFn";
import AddShopeeConfigModal from "../components/AddShopeeConfigModal.vue";

const { invoker: deleter } = removeConfigShopee({
	onSuccess(msg) {
		message.info(msg[0].toUpperCase() + msg.slice(1));
		invokeGetCfg();
		activeCfg.value = undefined;
	},
	onError(e) {
		message.error(JSON.stringify(e));
	},
});
const { invoker, pending: pendingUpdate } = editConfigShopee({
	onSuccess(msg) {
		message.info(msg[0].toUpperCase() + msg.slice(1));
		invokeGetCfg();
	},
	onError(e) {
		message.error(JSON.stringify(e));
	},
});
const { invoker: createConfigFn } = createConfigShopee({
	async onSuccess(msg) {
		message.info(msg[0].toUpperCase() + msg.slice(1));
		await invokeGetCfg();
		const last = config.value?.[0];
		if (last) {
			activeCfg.value = last.id;
		}
	},
});
const { config, invoker: invokeGetCfg } = useGetConfig();
const { errorString, pending, sellerTypes, shippingAgents } = useGetFilterConfigShopee();

const defaultForm = {
	ban_keywords: [],
	cities: [],
	exclude_cod: true,
	minimum_sold: 1,
	minimum_star: 4,
	minimum_stock: 10,
	name: "",
	price_ranges: [10000, 100000],
	seller_types: [],
	shippings: [],
	sort_by: "relevancy",
	id: -1,
};

const router = useRouter();
const openModalAdd = ref(false);
const form = ref<ShopeeConfigData>({ ...defaultForm });
const activeCfg = ref<number>();
const userCity = ref<string>();
const sortedTypes = [
	{
		label: "Terkait",
		value: "relevancy",
	},
	{
		label: "Terbaru",
		value: "ctime",
	},
	{
		label: "Terlaris",
		value: "sales",
	},
	{
		label: "Harga: Rendah ke Tinggi",
		value: "price_asc",
	},
	{
		label: "Harga: Tinggi ke Rendah",
		value: "price_desc",
	},
];

function onAddCity(_: KeyboardEvent) {
	const value = userCity.value;

	if (value) {
		if (!form.value.cities.includes(value)) {
			form.value.cities.push(value);
		}

		userCity.value = "";
	}
}

function onCloseCity(e: string) {
	form.value.cities = form.value.cities.filter((ct) => ct !== e);
}

watch(
	() => activeCfg.value,
	(cfgID) => {
		if (cfgID) {
			const selected = config.value?.filter((c) => c.id === cfgID)[0];
			if (selected) {
				form.value = { ...selected };
			} else {
				form.value = { ...defaultForm };
			}
		} else {
			form.value = { ...defaultForm };
		}
	}
);
</script>

<template>
	<div class="container">
		<div class="primer_container">
			<AddShopeeConfigModal
				@ok="
					(name) => {
						if (name) {
							createConfigFn({ ...form, name });
						}
					}
				"
				v-model:show="openModalAdd"
			/>
			<Alert
				show-icon
				type="error"
				:description="errorString"
				message="Error Get Config"
				v-if="errorString"
				closable
			></Alert>
			<Card title="Pengaturan Config Shopee" style="width: 100%" size="small">
				<template #extra>
					<Space v-show="pending">
						<Spin size="small" />
						<TypographyText type="secondary" style="font-weight: 400">
							Sedang memuat config dari shopee..
						</TypographyText>
					</Space>
				</template>
				<div class="card_filter">
					<Form
						:label-col="{ span: 6 }"
						:wrapper-col="{ span: 24 }"
						:label-wrap="true"
						:colon="false"
						size="small"
						layout="horizontal"
						label-align="left"
						style="width: 70%"
					>
						<FormItem label="Tipe Penjual">
							<Checkbox
								:checked="form.seller_types.includes(seller.value)"
								@update:checked="
									(c) => {
										if (c) {
											form.seller_types.push(seller.value);
										} else {
											form.seller_types = form.seller_types.filter(
												(s) => s !== seller.value
											);
										}
									}
								"
								v-for="seller in sellerTypes"
								:key="seller.value"
								>{{ seller.label }}</Checkbox
							>
						</FormItem>
						<FormItem label="Range Harga Produk">
							<InputGroup compact>
								<InputNumber
									style="width: 38%"
									:controls="false"
									addon-before="Rp."
									placeholder="10.000"
									:value="form.price_ranges[0] || 0"
									@update:value="
										(v) => {
											form.price_ranges[0] = v;
										}
									"
									:formatter="
										(value) =>
											`${value}`.replace(
												/\B(?=(\d{3})+(?!\d))/g,
												'.'
											)
									"
									:parser="(value) => value.replace(/\$\s?|(\.*)/g, '')"
								/>
								<InputNumber
									:value="form.price_ranges[1] || 0"
									@update:value="
										(v) => {
											form.price_ranges[1] = v;
										}
									"
									placeholder="100.000"
									:controls="false"
									addon-before="Rp."
									style="width: 38%"
									:formatter="
										(value) =>
											`${value}`.replace(
												/\B(?=(\d{3})+(?!\d))/g,
												'.'
											)
									"
									:parser="(value) => value.replace(/\$\s?|(\.*)/g, '')"
								/>
							</InputGroup>
						</FormItem>
						<FormItem label="Penilaian Penjual">
							<Rate v-model:value="form.minimum_star" allow-clear />
						</FormItem>
						<FormItem label="Terjual Minimal">
							<InputNumber
								:controls="false"
								placeholder="10"
								addon-after="Penjualan"
								v-model:value="form.minimum_sold"
							/>
						</FormItem>
						<FormItem label="Stock Tersedia Minimal">
							<InputNumber
								:controls="false"
								placeholder="100"
								addon-after="Item"
								v-model:value="form.minimum_stock"
							/>
						</FormItem>
						<FormItem label="Urutkan Pencarian">
							<Select
								v-model:value="form.sort_by"
								style="width: 50%"
								placeholder="Urutan pencarian"
							>
								<SelectOption
									v-for="sort in sortedTypes"
									:value="sort.value"
									>{{ sort.label }}</SelectOption
								>
							</Select>
						</FormItem>
						<FormItem label="Pilihan Jasa Kirim">
							<Row :gutter="[0, 10]">
								<Col v-for="ship in shippingAgents" :span="8">
									<Checkbox
										:checked="form.shippings.includes(+ship.value)"
										@update:checked="
											(c) => {
												if (c) {
													form.shippings.push(+ship.value);
												} else {
													form.shippings = form.shippings.filter(
														(sh) => sh !== +ship.value
													);
												}
											}
										"
										>{{ ship.label }}</Checkbox
									>
								</Col>
							</Row>
						</FormItem>
						<FormItem label="Pilihan Pembayaran">
							<Checkbox v-model:checked="form.exclude_cod">
								Kecualikan Cash On Delivery (COD)
							</Checkbox>
						</FormItem>
						<FormItem label="Ban Keyword">
							<Textarea
								:value="form.ban_keywords.join('\n')"
								@update:value="
									(str) => {
										form.ban_keywords = str.split('\n');
									}
								"
								:auto-size="{ minRows: 6, maxRows: 6 }"
							/>
						</FormItem>
					</Form>
					<Space size="middle" direction="vertical">
						<Card
							hoverable
							title="Pilih Nama Konfigurasi"
							size="small"
							type="inner"
						>
							<template #extra>
								<Button
									@click="openModalAdd = !openModalAdd"
									size="small"
									type="link"
									>Buat Baru</Button
								>
							</template>
							<InputGroup compact>
								<Select
									style="width: 100%"
									placeholder="Pilih Konfigurasi"
									v-model:value="activeCfg"
								>
									<SelectOption
										v-for="cfg in config"
										:value="cfg.id"
										::key="cfg.id"
										>{{ cfg.name }}</SelectOption
									>
								</Select>
							</InputGroup>
						</Card>
						<Alert show-icon type="info">
							<template #message>
								Pilih nama kota sesuai yang ada pada Shopee. <br />Contoh:
								"Kab. Surabaya" atau "Kota Surabaya" bisa kamu pilih
								dengan "surabaya", besar kecil karakter akan diabaikan.
							</template>
						</Alert>
						<Input
							addon-before="Nama Kota"
							placeholder="DKI Jakarta (Klik Enter untuk input)"
							@keydown.enter="onAddCity"
							v-model:value="userCity"
						></Input>
						<Row :gutter="[1, 7]">
							<Col v-for="city in form.cities" :key="city">
								<Tag
									color="cyan"
									@close="() => onCloseCity(city)"
									closable
									style="padding: 5px; font-size: 1em"
									>{{ city }}</Tag
								>
							</Col>
						</Row>
					</Space>
				</div>
				<Divider dashed style="margin-bottom: 15px" />
				<Space>
					<Button
						@click="
							() => {
								invoker({
									...form,
									ban_keywords: form.ban_keywords.filter((s) => s),
								});
							}
						"
						type="primary"
						:loading="pendingUpdate"
						:disabled="!form.id || form.id < 1"
						>Simpan</Button
					>
					<Button
						:disabled="!form.id || form.id < 1"
						type="primary"
						danger
						@click="
							() => {
								if (activeCfg) deleter({ id: activeCfg });
							}
						"
						>Hapus</Button
					>
					<Button
						type="default"
						@click="
							() => {
								router.back();
								message.destroy();
							}
						"
						>Kembali</Button
					>
				</Space>
			</Card>
		</div>
	</div>
</template>

<style lang="scss" scoped>
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
	.card_filter {
		display: flex;
		column-gap: 19px;

		> :first-child {
			width: 60% !important;
			@include media(">1366px") {
				width: 65% !important;
			}
		}
		> :nth-child(2) {
			flex: 1 !important;
		}
	}
}
</style>
