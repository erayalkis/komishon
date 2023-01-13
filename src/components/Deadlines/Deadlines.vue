<template>
  Deadlines
  <div>
    {{ deadlines }}
  </div>
</template>
<script setup>
import { onMounted, computed } from "vue";
import { useStore } from "vuex";
const { dispatch, state } = useStore();

const deadlines = computed(() => state.files.children);

onMounted(async () => {
  await dispatch("fetchDeadlinedFiles");
});
</script>
