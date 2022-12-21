<template>
  <div class="flex w-full bg-red-300 p-3">
    <input placeholder="search..." class="mx-auto" />
    <h1 class="ml-auto mr-3"><img :src="BellRegular" /></h1>
  </div>
</template>
<script setup>
import BellRegular from "@/assets/BellRegular.svg?url";
</script>
