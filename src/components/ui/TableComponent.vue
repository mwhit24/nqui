<template>

	<div class="p-2">
		<DataTable v-model:selection="selectedRows" :paginator="true" :resizableColumns="true" :rows="25"
		           :rowsPerPageOptions="[10,25,50]" :value="tableResults"
		           autoLayout
		           class="p-datatable-sm editable-cells-table" columnResizeMode="fit"
		           currentPageReportTemplate="Showing {first} to {last} of {totalRecords}"
		           dataKey="id"
		           editMode="cell"
		           paginatorTemplate="CurrentPageReport FirstPageLink PrevPageLink PageLinks NextPageLink LastPageLink RowsPerPageDropdown"
		           removableSort
		           responsiveLayout="scroll"
		           scrollHeight="400px"
		           selectionMode="multiple" showGridlines @cell-edit-complete="onCellEditComplete">
			<Column headerStyle="width: 3em" selectionMode="multiple"></Column>
			<Column v-for="col of columns" :key="col.field" :field="col.field" :header="col.header" :sortable="true"
			        class="text-xs"
			>
				<template #editor="{ data, field }">
					<InputText v-model="data[field]" class="p-inputtext-sm"/>
				</template>
			</Column>
		</DataTable>
	</div>
</template>

<script lang="ts" setup>
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import InputText from "primevue/inputtext";

import {invoke} from "@tauri-apps/api/tauri";
import type {Ref} from 'vue'
import {onMounted, ref} from "vue";

const props = defineProps({
	table: {
		type: String,
		required: true
	}
})

const columns: Ref<Record<string, string>[]> = ref([]);
const selectedRows = ref([]);
const tableResults = ref();
const rows: Ref<Record<string, string>[]> = ref([]);

onMounted(() => {
	if (props.table) {
		setupTableData();
	}
})

async function setupTableData(): Promise<void> {
	const tableData = JSON.parse(await invoke("fetch_related_table", {tableName: props.table}));
	if (tableData) {
		tableResults.value = tableData.results[0].rows;

		const columnsFromTableResults = tableResults.value.reduce((arr: any, o: {}) => {
			return Object.keys(o).reduce((a, k) => {
				if (a.indexOf(k) == -1) a.push(k);
				return a;
			}, arr)
		}, []);
		rows.value = tableResults.value;

		columnsFromTableResults.forEach((column: string) => {
			columns.value.push({
				field: column, header: column
			})
		})
	}
}

const onCellEditComplete = (event: { preventDefault?: any; data?: any; newValue?: any; field?: any; }) => {
	event.data[event.field] = event.newValue;
};
</script>
