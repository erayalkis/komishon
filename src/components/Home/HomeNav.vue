<template>
  <div
    class="flex w-full border-b border-gray-300 p-3.5 px-5"
    style="max-height: 10%"
  >
    <FilePath class="w-1/3" />
    <div class="relative mx-auto w-1/3 text-gray-800">
      <input
        @change="search"
        placeholder="Search"
        class="w-full rounded-md p-1 indent-10 bg-gray-200 hover:bg-gray-300 transition duration-300 ease-out"
      />
      <img :src="Search" class="w-5 h-5 absolute top-1.5 left-2" />
    </div>

    <div class="flex items-center justify-end w-1/3 ml-auto gap-5">
      <img
        @click="importFolder"
        :src="Upload"
        class="cursor-pointer w-5 h-5"
        title="Import a file"
      />
      <img
        :src="Grid"
        class="w-5 h-5 cursor-pointer"
        title="Grid View"
        @click="setViewStyle('grid')"
      />
      <img
        :src="List"
        class="w-5 h-5 cursor-pointer"
        title="List View"
        @click="setViewStyle('list')"
      />

      <img
        :src="Bell"
        class="w-5 h-5 cursor-pointer ml-4"
        @click="toggleViewNotifs"
      />

      <NotificationsDiv v-if="viewNotifs" />
    </div>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { ref } from "vue";
import FilePath from "..//FilePath/FilePath.vue";
import Search from "@/assets/Search.svg?url";
import Upload from "@/assets/Upload.svg?url";
import Grid from "@/assets/Grid.svg?url";
import List from "@/assets/List.svg?url";
import Bell from "@/assets/Bell.svg?url";
import NotificationsDiv from "../Notifications/NotificationsDiv.vue";

const { dispatch, commit } = useStore();

const viewNotifs = ref(false);

const toggleViewNotifs = () => (viewNotifs.value = !viewNotifs.value);

async function importFolder() {
  await dispatch("selectFolder");
}

async function setViewStyle(style) {
  commit("setViewStyle", style);
}

async function debugNotif() {
  dispatch("createDebugNotif");
}

async function search(v) {
  const input = v.target.value;
  await dispatch("searchByName", input);
}
</script>
