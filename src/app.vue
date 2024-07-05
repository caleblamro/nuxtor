<template>
	<Html min-h-screen>
		<Body overflow-x-hidden bg-dark-800 text-white>
			<NuxtLayout>
				<NuxtPage />
			</NuxtLayout>
		</Body>
	</Html>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';


onMounted(() => {
  invoke<string>('greet', { name: 'Caleb' }).then((message) => {
    console.log(message)
  })

  invoke<number>('add', { a: 1, b: 1 }).then((message) => {
    console.log(`1 + 1 = ${message}`)
  })
})

</script>

<style lang="scss">
	html {
		-webkit-tap-highlight-color: transparent;
		scroll-behavior: smooth;
	}
	img {
		-webkit-user-drag: none;
		@apply select-none;
	}

	// Transitions
	.page-enter-active,
	.page-leave-active {
		@apply transition-opacity ease-in-out duration-300;
	}
	.layout-enter-active,
	.layout-leave-active {
		@apply transition-opacity ease-in-out duration-500;
	}
	.page-enter-from,
	.page-leave-to,
	.layout-enter-from,
	.layout-leave-to {
		@apply opacity-0;
	}
</style>
