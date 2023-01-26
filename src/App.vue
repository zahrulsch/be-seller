<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { onMounted } from "vue";

onMounted(async () => {
	invoke("run_migration");
});
</script>

<template>
	<RouterView v-slot="{ Component }">
		<Transition mode="out-in">
			<component :is="Component"></component>
		</Transition>
	</RouterView>
</template>

<style>
.v-enter-active,
.v-leave-active {
	transition: 0.1s linear;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
}
</style>
