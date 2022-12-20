<template>
  <div class="w-32 h-32 border border-black cursor-pointer" @click="goTo(data)">
    {{ data.file_name }}
  </div>
</template>
<script setup>
import { useStore } from "vuex";
const { dispatch } = useStore();

defineProps({
  data: Object,
});

async function goTo(dir, idx = null) {
  dispatch("navigateTo", { dir, idx });
}
</script>
