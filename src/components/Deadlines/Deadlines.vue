<template>
  <template v-for="(val, key) in deadlines">
    {{ key }}
    <template v-for="file in val">
      {{ file.file_name }}
    </template>
  </template>
</template>
<script setup>
import { onMounted, computed } from "vue";
import { useStore } from "vuex";
const { dispatch, state } = useStore();

const deadlines = computed(() => state.deadlines.deadlines);

onMounted(async () => {
  await dispatch("loadDeadlines");
});
</script>
