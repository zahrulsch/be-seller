<script setup lang="ts">
import {
	Card,
	Divider,
	TypographyParagraph,
	TypographyTitle,
	Space,
} from "ant-design-vue";
import { ref } from "vue";
import config from "../assets/process.png";
import keyword from "../assets/keyword.png";
import category from "../assets/category.png";
import market from "../assets/market.png";
import bigseller from "../assets/bigseller.png";
import list from "../assets/list.png";
import url from "../assets/url.png";
import inpt from "../assets/input.png";
import upload from "../assets/upload.png";
import GrabByKeyword from "../components/GrabByKeyword.vue";
import BigSellerLoginModal from "../components/BigSellerLoginModal.vue";
import ProfileList from "../components/ProfileList.vue";
import CollectionFromDb from "../components/CollectionFromDb.vue";

const openGrabByKwd = ref(false);
const openBigSellerLogin = ref(false);
const openListProfile = ref(false);
const openCollFromDb = ref(false);

const menus = [
	{
		title: "Pengaturan Config",
		desc: "Atur konfigurasi pribadimu sebelum memulai semua",
		cover: config,
		uri: "/config",
		active: true,
	},
	{
		title: "Pencarian Keyword",
		desc: "Kamu bisa mencari produk dengan banyak kata kunci di sini",
		cover: keyword,
		uri: "",
		active: true,
		action: () => (openGrabByKwd.value = !openGrabByKwd.value),
	},
	{
		title: "Pencarian Kategori",
		desc: "Kamu bisa mencari produk dengan banyak kata kategori di sini",
		cover: category,
		uri: "",
		active: false,
	},
	{
		title: "Ambil Produk Toko",
		desc: "Ambil semua produk yang ada di toko tertentu",
		cover: market,
		uri: "",
		active: false,
	},
	{
		title: "Ambil Berdasarkan URL",
		desc: "Kumpulkan data dari daftar URL",
		cover: url,
		uri: "",
		active: true,
	},
	{
		title: "Kelola Koleksi",
		desc: "Kelola koleksi dari hasil grab yang sudah ada",
		cover: inpt,
		uri: "",
		active: true,
		action: () => (openCollFromDb.value = !openCollFromDb.value),
	},
];

const secondaryMenus = [
	{
		title: "Tambah Akun Big Seller",
		desc: "Buat akun Big Seller dan tambahkan di sini",
		cover: bigseller,
		uri: "",
		active: true,
		action: () => (openBigSellerLogin.value = !openBigSellerLogin.value),
	},
	{
		title: "List Akun Big Seller",
		desc: "Buka menu untuk dapatakan informasi lengkap tentang profile Big Seller",
		cover: list,
		uri: "",
		active: true,
		action: () => (openListProfile.value = !openListProfile.value),
	},
	{
		title: "Uploader Big Seller",
		desc: "Gunakan uploader Big Seller untuk upload ke semua atau beberapa profile",
		cover: upload,
		uri: "/bs-upload",
		active: true,
	},
];
</script>

<template>
	<div class="container">
		<div class="primer_container">
			<CollectionFromDb v-model:show="openCollFromDb" />
			<ProfileList v-model:show="openListProfile" />
			<GrabByKeyword v-model:show="openGrabByKwd" />
			<BigSellerLoginModal v-model:show="openBigSellerLogin" />
			<TypographyTitle :level="5">Daftar Menu</TypographyTitle>
			<Space style="flex-wrap: wrap; width: 100%" size="middle">
				<RouterLink :to="menu.uri" v-for="menu in menus" :key="menu.title">
					<Card
						:class="!menu.active && 'menu_card_off'"
						:hoverable="menu.active"
						:title="menu.title"
						size="small"
						class="menu_card"
						@click="menu.action"
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
				</RouterLink>
			</Space>
			<Divider dashed />
			<TypographyTitle :level="5">Fitur Big Seller</TypographyTitle>
			<Space size="middle">
				<RouterLink
					:to="menu.uri"
					v-for="menu in secondaryMenus"
					:key="menu.title"
				>
					<Card
						:title="menu.title"
						:class="!menu.active && 'menu_card_off'"
						:hoverable="menu.active"
						size="small"
						class="menu_card"
						@click="menu.action"
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
				</RouterLink>
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
					padding: 22px;
					@include media(">1366px") {
						padding: 37px;
					}
				}
			}
			&_off {
				width: 190px !important;

				@include media(">1366px") {
					width: 210px !important;
				}
				.image {
					transition: 0.1s ease-in-out;
					padding: 24px;
					border-radius: 10px;
					overflow: hidden;
					filter: grayscale(1) !important;
					@include media(">1366px") {
						padding: 40px;
					}
				}
				&:hover {
					.image {
						transition: 0.1s ease-in-out;
						padding: 22px;
						border-radius: 10px;
						overflow: hidden;
						filter: grayscale(1) !important;
						@include media(">1366px") {
							padding: 40px;
						}
					}
				}
			}
		}
	}
	.image {
		transition: 0.1s ease-in-out;
		padding: 24px;
		border-radius: 10px;
		overflow: hidden;
		filter: grayscale(0.23);

		@include media(">1366px") {
			padding: 40px;
		}
	}
}
</style>
