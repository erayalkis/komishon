<template>
  <div
    class="h-full min-h-screen w-56 bg-gray-300 text-left p-4 gap-5 flex flex-col"
  >
    <NavHeader />
    <h1><router-link to="/">Home</router-link></h1>
    <h1><router-link to="/favorites">Favorites</router-link></h1>
    <h1><router-link to="/deadlines">Deadline Report</router-link></h1>
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
