<template>
	<div class="flex flex-row justify-end py-2 px-4">
		<span class="isolate inline-flex rounded-md shadow-sm">
		    <button type="button" @click="openLeftDrawer" class="relative inline-flex items-center rounded-l-md border border-gray-300 bg-white px-2 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500">
		      <span class="sr-only">Previous</span>
		      <ChevronLeftIcon class="h-5 w-5" aria-hidden="true" />
		    </button>
		    <button type="button" class="relative -ml-px inline-flex items-center rounded-r-md border border-gray-300 bg-white px-2 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500">
		      <span class="sr-only">Next</span>
		      <ChevronRightIcon class="h-5 w-5" aria-hidden="true" />
		    </button>
        </span>
	</div>
</template>

<script lang="ts" setup>
import {ChevronLeftIcon, ChevronRightIcon} from "@heroicons/vue/24/solid";
import {useLayoutStore} from "../../stores/layout";

const layoutStore = useLayoutStore();

function openLeftDrawer() {
	layoutStore.setLeftDrawerOpen(true);
}

function openRightDrawer() {
	layoutStore.setRightDrawerOpen(true);
}
</script>
