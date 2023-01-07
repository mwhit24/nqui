<template>
    <div id="app-container" class="flex flex-row w-full justify-between px-4 py-2">
        <div id="left-drawer" class="border-r-2 border-black">
            <span>Here</span>
        </div>
        <div id="main-panel" class="">
            <Connection />
        </div>
        <div id="right-drawer">
            <span>Here right</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import Connection from './components/Connection.vue';
</script>


