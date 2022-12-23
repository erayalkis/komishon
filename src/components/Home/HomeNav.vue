<template>
  <div class="flex flex-col h-fit w-full">
    <div class="flex w-full border-b border-gray-900 p-2 px-5">
      <FilePath />
      <div class="relative mx-auto w-4/12 text-gray-900">
        <input
          placeholder="search..."
          class="w-full rounded-md p-1 indent-10"
        />
        <img :src="Search" class="w-5 h-5 absolute top-1.5 left-2" />
      </div>

      <div class="flex items-center ml-auto gap-5">
        <img
          @click="importFolder"
          :src="Upload"
          class="cursor-pointer w-5 h-5"
          title="Import a file"
        />
        <img :src="List" class="w-5 h-5 cursor-pointer" title="List View" />
        <img :src="Grid" class="w-5 h-5 cursor-pointer" title="Grid View" />
      </div>
    </div>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import FilePath from "..//FilePath/FilePath.vue";
import Search from "@/assets/Search.svg?url";
import Upload from "@/assets/Upload.svg?url";
import Grid from "@/assets/Grid.svg?url";
import List from "@/assets/List.svg?url";
const { dispatch } = useStore();

async function importFolder() {
  await dispatch("selectFolder");
}
</script>
