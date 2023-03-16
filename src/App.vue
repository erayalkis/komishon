<template>
  <TagModal v-if="viewSettings.tag" />
  <DeadlineModal v-if="viewSettings.deadline" />
  <div class="bg-stone-50 h-screen overflow-y-hidden flex">
    <Navbar />
    <div class="w-full">
      <HomeNav ref="homeNav" />
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { appDataDir } from "@tauri-apps/api/path";
import { onBeforeMount, onUnmounted, ref, computed } from "vue";
import { useStore } from "vuex";
import { listen } from "@tauri-apps/api/event";
import Navbar from "./components/Nav/Navbar.vue";
import TagModal from "@/components/Modals/TagModal.vue";
import DeadlineModal from "@/components/Modals/DeadlineModal.vue";
import HomeNav from "./components/Home/HomeNav.vue";

const { commit, dispatch, state } = useStore();
const unlisteners = ref([]);
const viewSettings = computed(() => state.modals.view);

const setupListeners = async () => {
  const unlistenRename = await listen("file-rename", (event) => {
    const { id, name, path } = event.payload;
    commit("updateFile", { id, name, path });
  });
  const unlistenRemove = await listen("file-remove", (event) => {
    const { path } = event.payload;
    commit("removeFile", { path });
  });
  const unlistenCreate = await listen("file-create", (event) => {
    commit("addFileToChildren", { file: event.payload });
  });
  const unlistenAddNotif = await listen("notif-create", (event) => {
    commit("addNotification", event.payload);
  });
  const unlistenRemoveNotif = await listen("notif-remove", (event) => {
    commit("removeNotification", event.payload);
  });

  unlisteners.value.push(unlistenRename);
  unlisteners.value.push(unlistenRemove);
  unlisteners.value.push(unlistenCreate);
  unlisteners.value.push(unlistenAddNotif);
  unlisteners.value.push(unlistenRemoveNotif);
};

onBeforeMount(async () => {
  await invoke("create_db_if_not_exists");
  await invoke("watch_base_dirs");
  await setupListeners();

  console.log(await appDataDir());
  dispatch("loadSettings");
  dispatch("loadInitialDirs");
  dispatch("loadNotifications");
});

onUnmounted(async () => {
  unlisteners.value.forEach((unlistener) => unlistener());
});
</script>
