<template>
	<TransitionRoot as="template" :show="leftDrawerOpen">
		<Dialog as="div" class="relative z-40" @close="leftDrawerOpen = false">
			<TransitionChild as="template" enter="transition-opacity ease-linear duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="transition-opacity ease-linear duration-300" leave-from="opacity-100" leave-to="opacity-0">
				<div class="fixed inset-0 bg-gray-600 bg-opacity-75" />
			</TransitionChild>

			<div class="fixed inset-0 z-40 flex">
				<TransitionChild as="template" enter="transition ease-in-out duration-300 transform" enter-from="-translate-x-full" enter-to="translate-x-0" leave="transition ease-in-out duration-300 transform" leave-from="translate-x-0" leave-to="-translate-x-full">
					<DialogPanel class="relative flex w-full max-w-xs flex-1 flex-col bg-white focus:outline-none">
						<TransitionChild as="template" enter="ease-in-out duration-300" enter-from="opacity-0" enter-to="opacity-100" leave="ease-in-out duration-300" leave-from="opacity-100" leave-to="opacity-0">
							<div class="absolute top-0 right-0 -mr-12 pt-2">
								<button type="button" class="ml-1 flex h-10 w-10 items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white" @click="leftDrawerOpen = false">
									<span class="sr-only">Close sidebar</span>
									<XMarkIcon class="h-6 w-6 text-white" aria-hidden="true" />
								</button>
							</div>
						</TransitionChild>
						<div class="h-0 flex-1 overflow-y-auto pt-5 pb-4">
							<div class="flex flex-shrink-0 items-center px-4 border-black text-black border-b">
								<h2 class="text-lg">Connections</h2>
							</div>
							<div class="space-y-1 px-2">
								<span class="text-black" v-for="item in connections" :key="item.connectionString">
									{{ item.connectionString }}
								</span>
							</div>
						</div>
					</DialogPanel>
				</TransitionChild>
				<div class="w-14 flex-shrink-0" aria-hidden="true">
					<!-- Force sidebar to shrink to fit close icon -->
				</div>
			</div>
		</Dialog>
	</TransitionRoot>
</template>

<script lang="ts" setup>
import {Dialog, DialogPanel, TransitionChild, TransitionRoot} from "@headlessui/vue";
import {XMarkIcon} from "@heroicons/vue/24/solid";
import {useLayoutStore} from "../../stores/layout";
import {storeToRefs} from "pinia";
import {useConnectionStore} from "../../stores/connection";

const layoutStore = useLayoutStore();
const connectionsStore = useConnectionStore();
const { leftDrawerOpen } = storeToRefs(layoutStore);
const { connections } = storeToRefs(connectionsStore);
</script>
