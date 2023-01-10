<template>
  <FilesList v-if="viewStyle === 'list'" />
  <Files v-else />
</template>
<script setup>
import Files from "../Files/Files.vue";
import FilesList from "../Files/FilesList.vue";
import { useStore } from "vuex";
import { computed } from "vue";
const { state } = useStore();

const viewStyle = computed(() => state.settings.preferredViewMode);
</script>
