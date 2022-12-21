<template>
  <div class="flex flex-wrap gap-5 overflow-y-auto px-5 py-6">
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
