<template>
  <div class="flex bg-gray-200">
    <template v-for="(dir, idx) in paths">
      <a @click="goTo(dir, idx)">
        <div>{{ dir.file_name }}</div>
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
