<template>
  <div
    class="h-full min-h-screen w-56 bg-gray-200 text-left gap-5 flex flex-col"
  >
    <NavHeader class="text-center p-3" />
    <router-link to="/">
      <h1
        class="text-xl text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-white transition duration-200 ease-out cursor-pointer"
      >
        Home
      </h1>
    </router-link>
    <router-link to="/favorites">
      <h1
        class="text-xl text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-white transition duration-200 ease-out cursor-pointer"
      >
        Favorites
      </h1>
    </router-link>
    <router-link to="/deadlines">
      <h1
        class="text-xl text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-white transition duration-200 ease-out cursor-pointer"
      >
        Deadline Report
      </h1>
    </router-link>
    <div class="mt-auto mx-auto italic opacity-25">
      <h1>Komishon build {{ tauriVersion }}</h1>
      <h1>Early alpha release</h1>
    </div>
  </div>
</template>
<script setup>
import NavHeader from "./NavHeader.vue";
import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";

const tauriVersion = ref(null);
onMounted(async () => {
  tauriVersion.value = await getVersion();
});
</script>
