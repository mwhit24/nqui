<template>
    <div class="p-2">
        <table class="w-full divide-y divide-gray-300">
        <thead class="bg-red-500 flex w-full">
            <div class="w-full flex">  
                <tr v-for="column in columns">
                <th class="py-3.5 pl-4 pr-3 text-sm font-semibold text-gray-900 sm:pl-6" scope="row">{{ column }}</th>
        
                <!-- <th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Title</th>
                <th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Email</th>
                <th class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900" scope="col">Role</th>
                <th class="relative py-3.5 pl-3 pr-4 sm:pr-6" scope="col">
                    <span class="sr-only">Edit</span>
                </th> -->
                </tr>  
            </div>
        </thead>
        <tbody class="bg-white w-full">
        <tr v-for="row in rows" :key="row.id">
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{ row[columns[0]] }}</td>
            <td class="px-3 py-4 text-sm text-gray-500">{{ row[columns[1]] }}</td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{{row.age }}</td>
            
        </tr>
        </tbody>
    </table>
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
const rows: Ref<Record<string, string>[]> = ref([]);

onMounted(() => {
	if (props.table) {
        console.log(props.table)
		setupTableData();
	}
})

async function setupTableData(): Promise<void> {
	const tableData = JSON.parse(await invoke("fetch_related_table", { tableName: props.table }));
	if (tableData) {
        console.log(`table`, tableData)
		const tableResults = tableData.results[0].rows;
        rows.value = tableResults;
		for (let row of tableResults) {
            const columnNames = Object.keys(row);
            columns.value = columnNames;
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
