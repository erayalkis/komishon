<template>
  <div
    class="bg-slate-400 h-screen overflow-y-hidden flex"
    @contextmenu.prevent="$refs.menu.open"
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
import { onBeforeMount, onBeforeUnmount } from "vue";
import { useStore } from "vuex";
import { listen } from "@tauri-apps/api/event";
import Navbar from "./components/Nav/Navbar.vue";
import ContextMenu from "./components/ContextMenu/ContextMenu.vue";

const { dispatch } = useStore();

onBeforeMount(async () => {
  await invoke("create_db_if_not_exists");
  await invoke("watch_base_dirs");
  await dispatch("loadInitialDirs");

  const unlistenRename = listen("file-rename", (event) => {
    console.log(event);
  });

  const unlistenRemove = listen("file-remove", (event) => {
    console.log(event);
  });

  const unlistenCreate = listen("file-create", (event) => {
    console.log(event);
  });

  onBeforeUnmount(() => {
    unlistenRename();
    unlistenRemove();
    unlistenCreate();
  });
});
</script>
