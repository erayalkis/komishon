<template>
  <div class="flex gap-3">
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
// const paths = computed(() => state.files.paths);
</script>
