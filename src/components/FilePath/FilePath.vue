<template>
  <div class="flex">
    <template v-for="(dir, idx) in paths">
      <a class="cursor-pointer" @click="goTo(dir, idx)">
        <div v-if="idx != paths.length - 1" class="flex">
          {{ dir.file_name }}
          <p class="px-2">></p>
        </div>
        <div v-else>{{ dir.file_name }}</div>
      </a>
    </template>
  </div>
</template>
<script setup>
import { computed } from "vue";
import { useStore } from "vuex";
const { state, dispatch } = useStore();
const paths = computed(() => state.files.paths);

function goTo(dir, idx = null) {
  dispatch("navigateTo", { dir, idx });
}
</script>
