<template>
  <div @click="view">View in file explorer</div>
</template>
<script setup>
import { openFileWithShell } from "@/api/shell/actions.js";

const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});

const view = () => {
  openFileWithShell(props.targetObj.path);
};
</script>
