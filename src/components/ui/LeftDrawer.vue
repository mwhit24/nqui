<template>
	<TransitionRoot :show="true" appear as="div">
		<div class="fixed inset-0 z-40 flex">
			<TransitionChild as="template" enter="transition ease-in-out duration-300 transform" enter-from="-translate-x-full" enter-to="translate-x-0" leave="transition ease-in-out duration-300 transform" leave-from="translate-x-0" leave-to="-translate-x-full">

			</TransitionChild>
			<div aria-hidden="true" class="w-14 flex-shrink-0">
				<!-- Force sidebar to shrink to fit close icon -->
			</div>
		</div>
	</TransitionRoot>
</template>
<script lang="ts" setup>
import {DialogPanel, TransitionChild, TransitionRoot} from "@headlessui/vue";
</script>
