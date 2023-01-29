<template>
	<Layout/>
</template>

<script lang="ts" setup>
import Layout from './components/ui/Layout.vue';</script>
	<ButtonBar />
    <div id="app-container" class="flex flex-row w-full justify-center px-4 py-2">
	    <div id="left-drawer">
		    <LeftDrawer />
	    </div>
        <div id="main-panel" class="w-full min-h-full">
            <Main />
        </div>
        <div id="right-drawer">
            <RightDrawer />
        </div>
    </div>
</template>

<script setup lang="ts">
import Main from "./components/layout/Main.vue";
import LeftDrawer from "./components/layout/LeftDrawer.vue";
import RightDrawer from "./components/layout/RightDrawer.vue";
import ButtonBar from "./components/layout/ButtonBar.vue";
</script>
>>>>>>> 19078fe1b39ccd0e1b7eb15a686534ef625f8628


