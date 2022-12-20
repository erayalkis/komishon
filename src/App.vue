<template>
  <div class="bg-slate-400 h-screen w-screen flex">
    <Navbar />
    <div class="w-8/12">
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { appDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { onBeforeMount } from "vue";
import { useStore } from "vuex";
import Navbar from "./components/Nav/Navbar.vue";
const { dispatch } = useStore();

onBeforeMount(async () => {
  const appData = await appDataDir();
  await invoke("create_db_if_not_exists", { to: `${appData}/entries.db` });
  await dispatch("loadInitialDirs");
});
</script>
