<template>
	<div class="overflow-hidden rounded-lg bg-white shadow">
		<div class="px-4 py-5 sm:p-6">
			<div>
				<label for="email" class="block text-sm font-medium text-gray-700">Enter nqlite connection address</label>
				<div class="mt-1">
					<input v-model="connectionString" type="text" name="connection-string" id="connection-string" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" placeholder="https://example:4001" />
				</div>
			</div>
			<div class="flex justify-end mt-2">
				<button @click="saveConnection" type="button" class="flex items-end justify-center rounded border border-transparent bg-indigo-600 px-2.5 py-1.5 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
					Connect
				</button>
			</div>

		</div>


	</div>
</template>

<script setup lang="ts">

import {useConnectionStore} from "../stores/connection";
import {storeToRefs} from "pinia";

const connectionStore = useConnectionStore();
const { connections, connectionString } = storeToRefs(connectionStore);

function saveConnection() {
	if (connectionString.value) {
		connectionStore.setConnectionString(connectionString.value)
		connectionStore.updateConnections({connectionName: 'My Connection', connectionString: connectionString.value})
	}

}

</script>
