<script lang="ts" setup>
import {
	Card,
	Form,
	FormItem,
	Checkbox,
	InputGroup,
	InputNumber,
	Rate,
	TypographyText,
	CheckboxGroup,
	Row,
	Col,
	Input,
	Space,
	Select,
	Textarea,
	Button,
	Divider,
} from "ant-design-vue";
import { useRouter } from "vue-router";

const router = useRouter();

const sellerType = [
	{
		label: "Star",
		value: 7,
	},
	{
		label: "Star+",
		value: 6,
	},
	{
		label: "Shopee Mall",
		value: 5,
	},
	{
		label: "Dikelola Shopee",
		value: 4,
	},
];

const shippingAgents = [
	{ name: "Instant - 2 Jam" },
	{ name: "Reguler" },
	{ name: "Hemat" },
	{ name: "Same Day" },
	{ name: "Kargo" },
	{ name: "Next Day" },
	{ name: "Instant Car" },
	{ name: "Ambil di Tempat" },
];

const addresses = [
	{ location: "Jabodetabek" },
	{ location: "Jawa Timur" },
	{ location: "DI Yogyakarta" },
	{ location: "Sulawesi Selatan" },
	{ location: "Kalimantan Selatan" },
	{ location: "DKI Jakarta" },
	{ location: "Jawa Barat" },
	{ location: "Jawa Tengah" },
	{ location: "Kepulauan Riau" },
	{ location: "Banten" },
	{ location: "Sumatera Utara" },
	{ location: "Bali" },
	{ location: "Lampung" },
	{ location: "Sumatera Selatan" },
	{ location: "Riau" },
	{ location: "Jambi" },
	{ location: "Kalimantan Timur" },
	{ location: "Kalimantan Barat" },
	{ location: "Dalam Negeri" },
	{ location: "Luar Negeri" },
];
</script>

<template>
	<div class="container">
		<div class="primer_container">
			<Card style="width: 100%" size="small" title="Pengaturan Config Shopee">
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
							<Checkbox v-for="seller in sellerType" :key="seller.value">{{
								seller.label
							}}</Checkbox>
						</FormItem>
						<FormItem label="Range Harga Produk">
							<InputGroup compact>
								<InputNumber
									style="width: 38%"
									:controls="false"
									addon-before="Rp."
									placeholder="10.000"
								/>
								<InputNumber
									placeholder="100.000"
									:controls="false"
									addon-before="Rp."
									style="width: 38%"
								/>
							</InputGroup>
						</FormItem>
						<FormItem label="Penilaian Penjual">
							<Rate allow-clear allow-half />
						</FormItem>
						<FormItem label="Terjual Minimal">
							<InputNumber
								:controls="false"
								placeholder="10"
								addon-after="Penjualan"
							/>
						</FormItem>
						<FormItem label="Stock Tersedia Minimal">
							<InputNumber
								:controls="false"
								placeholder="100"
								addon-after="Item"
							/>
						</FormItem>
						<FormItem label="Urutkan Pencarian">
							<Select
								style="width: 50%"
								placeholder="Urutan pencarian"
								:options="[]"
							>
							</Select>
						</FormItem>
						<FormItem label="Pilihan Jasa Kirim">
							<CheckboxGroup>
								<Row :gutter="[0, 10]">
									<Col v-for="ship in shippingAgents" :span="8">
										<Checkbox>{{ ship.name }}</Checkbox>
									</Col>
								</Row>
							</CheckboxGroup>
						</FormItem>
						<FormItem label="Pilihan Pembayaran">
							<Checkbox type="secondary">
								<TypographyText type="warning"
									>Kecualikan Cash On Delivery (COD)</TypographyText
								>
							</Checkbox>
						</FormItem>
						<FormItem label="Ban Keyword">
							<Textarea :auto-size="{ minRows: 6, maxRows: 6 }" />
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
								<Button size="small" type="link">Buat Baru</Button>
							</template>
							<InputGroup compact>
								<Select
									style="width: 100%"
									placeholder="Pilih Konfigurasi"
								/>
							</InputGroup>
						</Card>
						<Input addon-before="Nama Kota" placeholder="DKI Jakarta"></Input>
						<Row :gutter="[8, 8]">
							<Col :span="12" v-for="address in addresses">
								<Checkbox>{{ address.location }}</Checkbox>
							</Col>
						</Row>
					</Space>
				</div>
				<Divider dashed style="margin-bottom: 15px" />
				<Space>
					<Button type="primary">Simpan</Button>
					<Button type="default" @click="() => router.back()">Batal</Button>
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
				width: 70% !important;
			}
		}
		> :nth-child(2) {
			flex: 1 !important;
		}
	}
}
</style>
