<script setup lang="ts">
import {
	Card,
	Divider,
	TypographyParagraph,
	TypographyTitle,
	Space,
} from "ant-design-vue";
import config from "../assets/process.png";
import keyword from "../assets/keyword.png";
import category from "../assets/category.png";
import market from "../assets/market.png";
import bigseller from "../assets/bigseller.png";

const menus = [
	{
		title: "Pengaturan Config",
		desc: "Konfigurasi yang bisa kamu atur seperti: filter grab, markup, dll",
		cover: config,
		uri: "/config",
	},
	{
		title: "Pencarian Keyword",
		desc: "Kamu bisa mencari produk dengan banyak kata kunci di sini",
		cover: keyword,
		uri: "",
	},
	{
		title: "Pencarian Kategori",
		desc: "Kamu bisa mencari produk dengan banyak kata kategori di sini",
		cover: category,
		uri: "",
	},
	{
		title: "Ambil Produk Toko",
		desc: "Ambil semua produk yang ada di toko tertentu",
		cover: market,
		uri: "",
	},
];

const secondaryMenus = [
	{
		title: "Tambah Akun Big Seller",
		desc: "Buat akun Big Seller dan tambahkan di sini",
		cover: bigseller,
	},
];
</script>

<template>
	<div class="container">
		<div class="primer_container">
			<TypographyTitle :level="5">Daftar Menu</TypographyTitle>
			<Space size="middle">
				<RouterLink :to="menu.uri" v-for="menu in menus" :key="menu.title">
					<Card :title="menu.title" size="small" hoverable class="menu_card">
						<template #cover>
							<img class="image" :src="menu.cover" :alt="menu.title" />
							<Divider
								style="margin-block: 0; margin-bottom: 4px"
								plain
								dashed
							/>
						</template>
						<Card.Meta>
							<template #description>
								<TypographyParagraph
									style="margin-bottom: 3px"
									type="secondary"
									:ellipsis="{
										rows: 2,
										tooltip: true,
									}"
									:content="menu.desc"
								></TypographyParagraph>
							</template>
						</Card.Meta>
					</Card>
				</RouterLink>
			</Space>
			<Divider dashed />
			<TypographyTitle :level="5">Fitur Big Seller</TypographyTitle>
			<Space size="middle">
				<Card
					v-for="menu in secondaryMenus"
					:key="menu.title"
					:title="menu.title"
					size="small"
					hoverable
					class="menu_card"
				>
					<template #cover>
						<img class="image" :src="menu.cover" :alt="menu.title" />
						<Divider
							style="margin-block: 0; margin-bottom: 4px"
							plain
							dashed
						/>
					</template>
					<Card.Meta>
						<template #description>
							<TypographyParagraph
								style="margin-bottom: 3px"
								type="secondary"
								:ellipsis="{
									rows: 2,
									tooltip: true,
								}"
								:content="menu.desc"
							></TypographyParagraph>
						</template>
					</Card.Meta>
				</Card>
			</Space>
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
		width: min-content;
		max-width: 1500px;
		padding: 20px;
		display: flex;
		column-gap: 7px;
		row-gap: 7px;
		flex-wrap: wrap;
		flex-direction: column;

		.menu_card {
			width: 190px !important;
			@include media(">1366px") {
				width: 210px !important;
			}
			&:hover {
				.image {
					filter: grayscale(0);
					padding: 12px;
					@include media(">1366px") {
						padding: 37px;
					}
				}
			}
		}
	}
	.image {
		transition: 0.1s ease-in-out;
		padding: 15px;
		border-radius: 10px;
		overflow: hidden;
		filter: grayscale(0.23);

		@include media(">1366px") {
			padding: 40px;
		}
	}
}
</style>
