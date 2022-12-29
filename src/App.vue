<template>
  <div class="bg-slate-400 h-screen overflow-y-hidden flex">
    <Navbar />
    <div class="w-full">
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onBeforeMount } from "vue";
import { useStore } from "vuex";
import Navbar from "./components/Nav/Navbar.vue";
const { dispatch } = useStore();

onBeforeMount(async () => {
  await invoke("create_db_if_not_exists");
  await dispatch("loadInitialDirs");
});
</script>
