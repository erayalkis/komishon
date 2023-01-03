<template>
  <div
    class="flex flex-wrap gap-2 px-3 py-3 overflow-y-auto"
    style="height: full"
  >
    <template v-for="child in children" :key="child.id">
      <File :data="child" />
    </template>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";
import File from "./File.vue";
const { state } = useStore();

const children = computed(() => state.files.children);
</script>
