<template>
  <div
    class="h-full min-h-screen w-56 bg-gray-200 text-left gap-5 flex flex-col"
  >
    <NavHeader class="text-center p-3" />
    <router-link to="/">
      <div
        class="nav-home flex items-center text-xl text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-zinc-100 transition duration-200 ease-out cursor-pointer"
      >
        <HomeSvg class="h-6 w-6 mr-1" />
        <h1>Home</h1>
      </div>
    </router-link>
    <router-link to="/favorites">
      <div
        class="text-xl flex text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-zinc-100 transition duration-200 ease-out cursor-pointer"
      >
        <HeartSvg class="h-6 w-6 mr-1" />
        <h1>Favorites</h1>
      </div>
    </router-link>
    <router-link to="/deadlines">
      <div
        class="flex items-center text-xl text-gray-900 w-full px-2 py-1.5 hover:bg-violet-500 hover:text-zinc-100 transition duration-200 ease-out cursor-pointer"
      >
        <CalendarSvg class="h-6 w-6 mr-1" />
        <h1>Deadline Report</h1>
      </div>
    </router-link>
    <div class="mt-auto mx-auto italic opacity-25">
      <h1>Komishon build {{ tauriVersion }}</h1>
      <h1>Early alpha release</h1>
    </div>
  </div>
</template>
<script setup>
import NavHeader from "./NavHeader.vue";
import Calendar from "@/assets/Calendar.svg?url";
import Heart from "@/assets/Heart.svg?url";
import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
import HomeSvg from "../../assets/svgComponents/HomeSvg.vue";
import HeartSvg from "../../assets/svgComponents/HeartSvg.vue";
import CalendarSvg from "../../assets/svgComponents/CalendarSvg.vue";

const tauriVersion = ref(null);

onMounted(async () => {
  tauriVersion.value = await getVersion();
});
</script>
