<template>
  <Files />
</template>
<script setup>
import { computed, onMounted } from "vue";
import { useStore } from "vuex";
import Files from "@/components/Files/Files.vue";
const { state, dispatch } = useStore();

onMounted(async () => {
  await dispatch("fetchFavoritedFiles");
});
</script>
