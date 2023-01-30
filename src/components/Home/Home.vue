<template>
  <template v-if="files.length > 0">
    <FilesList v-if="viewStyle === 'list'" />
    <Files v-else />
  </template>
  <template v-else>
    <div class="flex-col justify-center items-center w-full h-full">
      <div>
        <h1 class="text-5xl text-gray-900 w-max pt-20 mx-auto">
          Welcome to Komishon!
        </h1>
      </div>

      <h1 class="text-3xl text-gray-900 w-max mx-auto pt-6">
        Import a file to get started!
      </h1>
    </div>
  </template>
</template>
<script setup>
import Files from "../Files/Files.vue";
import FilesList from "../Files/FilesList.vue";
import { useStore } from "vuex";
import { computed } from "vue";
const { state } = useStore();

const viewStyle = computed(() => state.settings.preferredViewMode);
const files = computed(() => state.files.children);
</script>
