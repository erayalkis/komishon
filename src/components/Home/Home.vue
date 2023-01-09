<template>
  <button @click="testFunc">HELP</button>
  <button @click="testFunc2">HELP2</button>
  <HomeNav ref="homeNav" />
  <FilesList v-if="homeNav?.viewStyle === 'list'" />
  <Files v-else />
</template>
<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import Files from "../Files/Files.vue";
import FilesList from "../Files/FilesList.vue";
import HomeNav from "./HomeNav.vue";

const homeNav = ref(null);

const testFunc = async () => {
  const vals = await invoke("fetch_files_with_deadlines");
  console.log(JSON.parse(vals));
};

const testFunc2 = async () => {
  const vals = await invoke("fetch_favorited_files");
  console.log(JSON.parse(vals));
};
</script>
