<template>
	<div class="px-2 sm:px-10 md:px-2 lg:px-2">
		<div class="mt-2 flex flex-col">
			<div class="-my-2 mx-0 overflow-x-auto sm:-mx-6 lg:-mx-8">
				<div class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
					<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
						<table class="min-w-full divide-y divide-gray-300">
							<thead class="bg-gray-50">
							<tr>
								<th class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6" scope="col">Name</th>
								<th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Title</th>
								<th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Email</th>
								<th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Role</th>
								<th class="relative py-3.5 pl-3 pr-4 sm:pr-6" scope="col">
									<span class="sr-only">Edit</span>
								</th>
							</tr>
							</thead>
							<tbody class="bg-white">
							<tr v-for="(person, personIdx) in people" :key="person.email" :class="personIdx % 2 === 0 ? undefined : 'bg-gray-50'">
								<td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">{{ person.name }}</td>
								<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ person.title }}</td>
								<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ person.email }}</td>
								<td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ person.role }}</td>
								<td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
									<a class="text-indigo-600 hover:text-indigo-900" href="#"
									>Edit<span class="sr-only">, {{ person.name }}</span></a
									>
								</td>
							</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script lang="ts" setup>

import {invoke} from "@tauri-apps/api/tauri";
import {onMounted, ref} from "vue";
import type { Ref } from 'vue'

const props = defineProps({
	table: {
		type: String,
		required: true
	}
})

const columns: Ref<string[]> = ref([]);

onMounted(() => {
	if (props.table) {
		setupTableData();
	}
})

async function setupTableData(): Promise<void> {
	const tableData = JSON.parse(await invoke("fetch_related_table", { tableName: props.table }));
	if (tableData) {
		const tableResults = tableData.results[0].rows;
		for (let row in tableResults) {
			columns.value.push(tableResults[row]);
		}

		console.log(columns.value);
	}
}

// async function setCurrentTable(table: Record<string, string>): Promise<void> {
// 	currentTable.value = table.name;
// 	console.log(table)
// 	const tableData = JSON.parse(await invoke("fetch_related_table", { tableName: currentTable.value }));
// 	console.log(tableData)
// }
const people = [
	{ name: 'Lindsay Walton', title: 'Front-end Developer', email: 'lindsay.walton@example.com', role: 'Member' },
	{ name: 'Lindsay Walton', title: 'Front-end Developer', email: 'lindsay.walton@example.com', role: 'Member' }
	// More people...
]
</script>
