<template>
	<div class="h-full flex">
		<!-- Static sidebar for desktop -->
		<div class="inset-y-0 flex w-64 flex-col border-r border-gray-200 bg-gray-100 pt-2 pb-4">
			<!-- Sidebar component, swap this element with another sidebar if you like -->
			<div class="flex h-0 flex-1 flex-col overflow-y-auto pt-0">
				<!-- Sidebar Search -->
				<div class="px-3 pb-5 border-b">
					<label class="sr-only" for="search">Search</label>
					<div class="relative mt-1 rounded-sm shadow-sm">
						<div aria-hidden="true" class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
							<MagnifyingGlassIcon aria-hidden="true" class="mr-3 h-4 w-4 text-gray-400" />
						</div>
						<input id="search" class="block w-full rounded-md border-gray-300 pl-9 focus:border-red-500 focus:ring-red-500 sm:text-sm duration-200" name="search" placeholder="Search" type="text" />
					</div>
				</div>
				<!-- Navigation -->
				<nav class="mt-2 px-3">
					<div class="mt-1">
						<!-- Secondary navigation -->
						<h3 id="desktop-teams-headline" class="px-3 text-base font-normal text-black border-b w-full">Tables</h3>
						<div aria-labelledby="desktop-teams-headline" class="mt-1 space-y-1" role="group">
							<div v-if="tables.length > 0">
								<div v-for="table in tables" :key="table.name" class="cursor-pointer group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900 hover:duration-200"  @click="setCurrentTable(table)">
										<span class="truncate">{{ table.name }}</span>
								</div>
							</div>
							<div v-else>
								<h3 id="desktop-teams-headline" class="px-3 text-sm font-medium text-gray-500 text-center mt-10">No tables found!</h3>
							</div>

						</div>
					</div>
				</nav>
			</div>
		</div>
		<!-- Main column -->
		<div class="flex flex-col w-full">
			<main class="flex-1">
				<!-- Page title & actions -->
				<div class="border-b border-gray-200 px-4 py-3 flex items-baseline justify-center between">
					<div class="px-3 pb-2 w-full">
						<label class="sr-only" for="search">Run query</label>
						<div class="relative rounded-sm">
							<div aria-hidden="true" class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<MagnifyingGlassIcon aria-hidden="true" class="mr-3 h-4 w-4 text-gray-400" />
							</div>
							<input id="search" class="block w-full rounded-md border-gray-300 pl-9 focus:border-red-500 focus:ring-red-500 sm:text-sm duration-200" name="search" placeholder="Run query" type="text" />
						</div>
					</div>
					<div class="sm:ml-4 shadow-sm">
						<button class="sm:order-0 order-1 ml-3 inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 hover:duration-200 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 sm:ml-0" type="button">Run</button>
					</div>
				</div>
				<div v-if="currentTable">
					<Table :table="currentTable"/>
				</div>
			</main>
		</div>
	</div>
</template>

<script lang="ts" setup>
import {onMounted, Ref, ref} from 'vue'
import {
	Dialog,
	DialogPanel,
	Menu,
	MenuButton,
	MenuItem,
	MenuItems,
	TransitionChild,
	TransitionRoot,
} from '@headlessui/vue'
import { Bars3CenterLeftIcon, Bars4Icon, ClockIcon, HomeIcon, XMarkIcon } from '@heroicons/vue/24/outline'
import { ChevronRightIcon, ChevronUpDownIcon, EllipsisVerticalIcon, MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import {invoke} from "@tauri-apps/api/tauri";
import Table from "./TableComponent.vue";1


const tables: Ref<Record<string, string>[]> = ref([]);
const currentTable: Ref<string> = ref("");

onMounted(async () => {
	// Grab list of tables
await getTables();
})

async function setCurrentTable(table: Record<string, string>): Promise<void> {
	currentTable.value = tables.value.find(x => x.name === table.name)?.name as string;
}

async function getTables(): Promise<void> {
	const dbTables: any = JSON.parse(await invoke("get_tables"));
	console.log(dbTables.results)
	tables.value = dbTables.results[0].rows;

	console.log(tables.value);
}
</script>
