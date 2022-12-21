<template>
  <div class="flex flex-col h-fit w-full">
    <MainNav />
    <div class="flex w-full bg-red-200 p-2 px-5">
      <FilePath />
      <div class="flex items-center ml-auto gap-4">
        <h1 @click="importFolder">Import</h1>
        <h1>Table view</h1>
        <h1>Box view</h1>
      </div>
    </div>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import FilePath from "..//FilePath/FilePath.vue";
import MainNav from "../MainNav/MainNav.vue";
const { dispatch } = useStore();

async function importFolder() {
  await dispatch("selectFolder");
}
</script>
