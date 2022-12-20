<template>
  <div class="w-32 h-32 border border-black cursor-pointer" @click="goTo(data)">
    {{
      data.file_name.length > 10
        ? data.file_name.slice(0, 7) + "..."
        : data.file_name
    }}
  </div>
</template>
<script setup>
import { useStore } from "vuex";
const { dispatch } = useStore();

defineProps({
  data: Object,
});

function goTo(dir, idx = null) {
  if (!dir.is_dir) return;
  dispatch("navigateTo", { dir, idx });
}
</script>
