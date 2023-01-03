<template>
  <div @click="view">View in file explorer</div>
</template>
<script setup>
import { useStore } from "vuex";
import { openFileWithShell } from "@/api/shell/actions.js";

const { getters } = useStore();

const props = defineProps({
  targetId: {
    type: String,
    default: 0,
  },
});

const view = () => {
  const file = getters.getFile(props.targetId);
  openFileWithShell(file.path);
};
</script>
