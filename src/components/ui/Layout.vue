<template>
	<div class="h-full flex">
		<!-- Static sidebar for desktop -->
		<div class="inset-y-0 flex w-64 flex-col border-r border-gray-200 bg-gray-100 pt-2 pb-4">
			<!-- Sidebar component, swap this element with another sidebar if you like -->
			<div class="flex h-0 flex-1 flex-col overflow-y-auto pt-0">
				<!-- Sidebar Search -->
				<div class="px-3 pb-5 border-b shadow-none">
					<div class="mt-1">
						<input id="email"
						       class="block w-full rounded-md border-gray-300 shadow-sm sm:text-sm"
						       name="email"
						       placeholder="Search tables"
						       type="text"/>
					</div>
				</div>
				<!-- Navigation -->
				<nav class="mt-2 px-3">
					<div class="mt-1">
						<!-- Secondary navigation -->
						<h3 id="desktop-teams-headline" class="px-3 text-base font-normal text-black border-b w-full">
						Tables</h3>
						<div aria-labelledby="desktop-teams-headline" class="mt-1 space-y-1" role="group">
							<div v-if="tables.length > 0">
								<div v-for="table in tables" :key="table.name"
								     class="cursor-pointer group flex items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 hover:text-gray-900 hover:duration-200"
								     @click="setCurrentTable(table)">
									<span class="truncate">{{ table.name }}</span>
								</div>
							</div>
							<div v-else>
								<h3 id="desktop-teams-headline"
								    class="px-3 text-sm font-medium text-gray-500 text-center mt-10">No tables
								found!</h3>
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

							<input id="search"
							       class="block w-full rounded-md shadow-sm border-none hover:ring-1 hover:cursor-pointer sm:text-sm duration-200"
							       name="search" placeholder="Run query" type="text"/>
						</div>
					</div>
					<div class="sm:ml-4">
						<button
							class="sm:order-0 order-1 ml-3 inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 hover:duration-200 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 sm:ml-0"
							type="button">Run
						</button>
					</div>
				</div>
				<div class="p-2">
					<TabView v-if="tabs.length > 0" :activeIndex="activeTabIndex">
						<TabPanel v-for="(tab, tabIndex) in tabs" :key="tabIndex" class="text-xs">
							<template #header>
								<div class="flex justify-between items-center">
									<span class="text-sm">{{ tab.title }}</span>
									<Button class="p-button-text shadow-none hover:bg-gray-50"
									        @click="removeTab(tabIndex)">
										<CloseIcon class="h-3 w-3"/>
									</Button>
								</div>

							</template>
							<Table :table="tab.title"/>
						</TabPanel>
					</TabView>
				</div>

			</main>
		</div>
	</div>
</template>

<script lang="ts" setup>
import Button from "primevue/button";
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";
import {onMounted, Ref, ref, watch} from 'vue'
import {invoke} from "@tauri-apps/api/tauri";
import Table from "./TableComponent.vue";
import {Tab, useLayoutStore} from "../../stores/layout";
import {storeToRefs} from "pinia";
import {CloseIcon} from "@iconicicons/vue3";

const layoutStore = useLayoutStore();

const {currentTable, tabs} = storeToRefs(layoutStore);

const activeTabIndex = ref();
const tables: Ref<Record<string, string>[]> = ref([]);

onMounted(async () => {
	// Grab list of tables
	await getTables();
})

async function setCurrentTable(table: Record<string, string>): Promise<void> {
	const neededTable = tables.value.find(x => x.name === table.name);
	if (neededTable) {
		layoutStore.setCurrentTable(neededTable);
		const tab: Tab = {
			title: neededTable.name,
			table: neededTable
		}
		layoutStore.updateTabs(tab);
		activeTabIndex.value = tabs.value.findIndex(x => x.title === tab.title);
	}
}

function removeTab(tabIndex: number) {
	layoutStore.removeTab(tabIndex);
}

async function getTables(): Promise<void> {
	const dbTables: any = JSON.parse(await invoke("get_tables"));
	tables.value = dbTables.results[0].rows;
}

watch(tabs, () => {
	console.log(`tabs changed!`)
})
</script>
