<template>
  <div
    class="bg-slate-400 h-screen overflow-y-hidden flex"
    @contextmenu.prevent="ellaur"
  >
    <Navbar />
    <ContextMenu ref="menu" />
    <div class="w-full">
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { onBeforeMount, ref } from "vue";
import { useStore } from "vuex";
import Navbar from "./components/Nav/Navbar.vue";
import ContextMenu from "./components/ContextMenu/ContextMenu.vue";

const { dispatch } = useStore();
const menu = ref(null);

const ellaur = (e) => {
  menu.value.open(e);
};

onBeforeMount(async () => {
  await invoke("create_db_if_not_exists");
  await dispatch("loadInitialDirs");
});
</script>
