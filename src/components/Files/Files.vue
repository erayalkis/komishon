<template>
  <div
    class="flex flex-wrap gap-2 px-3 py-3 overflow-y-auto"
    style="max-height: 94%"
  >
    <template v-for="child in children">
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
