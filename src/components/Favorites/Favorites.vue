<template>
  <Files />
</template>
<script setup>
import { onMounted } from "vue";
import { useStore } from "vuex";
import Files from "@/components/Files/Files.vue";
const { dispatch } = useStore();

onMounted(async () => {
  await dispatch("fetchFavoritedFiles");
});
</script>
