<template>
  <div class="flex flex-col w-full">
    <MainNav />
    <div class="flex w-full bg-red-200 px-5">
      <FilePath />
      <div class="flex items-center ml-auto gap-4">
        <h1>Import</h1>
        <h1>Table view</h1>
        <h1>Box view</h1>
      </div>
    </div>
  </div>
</template>
<script setup>
import FilePath from "..//FilePath/FilePath.vue";
import MainNav from "../MainNav/MainNav.vue";
</script>
