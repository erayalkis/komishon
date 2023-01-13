<template>
  <div
    class="h-full min-h-screen w-56 bg-gray-300 text-left p-4 gap-5 flex flex-col"
  >
    <NavHeader />
    <h1><router-link to="/">Home</router-link></h1>
    <h1><router-link to="/favorites">Favorites</router-link></h1>
    <h1><router-link to="/deadlines">Deadline Report</router-link></h1>
    <div class="mt-auto">
      <h1>Komishon build v0.0.0</h1>
    </div>
  </div>
</template>
<script setup>
import NavHeader from "./NavHeader.vue";
</script>
