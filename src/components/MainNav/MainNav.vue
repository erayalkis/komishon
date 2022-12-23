<template>
  <div class="flex items-center w-full pt-5 p-3">
    <div class="relative mx-auto w-4/12 text-gray-900">
      <input placeholder="search..." class="w-full rounded-md p-1 indent-10" />
      <img :src="Search" class="w-5 h-5 absolute top-1.5 left-2" />
    </div>

    <h1 class="ml-auto mr-5 w-5 h-5 user-select-none">
      <img :src="BellRegular" />
    </h1>
  </div>
</template>
<script setup>
import BellRegular from "@/assets/BellRegular.svg?url";
import Search from "@/assets/Search.svg?url";
</script>
