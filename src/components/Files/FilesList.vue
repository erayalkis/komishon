<template>
  <div
    class="flex flex-col gap-2 px-3 py-3 overflow-y-auto"
    style="height: 96%"
  >
    <template v-for="child in children" :key="child.id">
      <FileRow :data="child" />
    </template>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";
import FileRow from "./FileRow.vue";
const { state } = useStore();

const children = computed(() => state.files.children);
</script>
