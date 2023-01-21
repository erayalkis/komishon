<template>
  <FilesList v-if="viewStyle === 'list'" />
  <Files v-else />
</template>
<script setup>
import { computed, onMounted } from "vue";
import { useStore } from "vuex";
import Files from "@/components/Files/Files.vue";
import FilesList from "../Files/FilesList.vue";
const { state, dispatch } = useStore();

const viewStyle = computed(() => state.settings.preferredViewMode);

onMounted(async () => {
  await dispatch("fetchFavoritedFiles");
});
</script>
