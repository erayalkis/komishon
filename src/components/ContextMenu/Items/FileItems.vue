<template>
  <div>{{ targetObj["file_name"] }}</div>
  <div>Add a tag</div>
  <div>Add a deadline</div>
</template>
<script setup>
const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});
</script>
