<script setup lang="ts"></script>

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
	transition: 0.15s linear;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
	filter: blur(5px);
}
</style>
