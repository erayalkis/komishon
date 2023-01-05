<template>
  <Tags :file="targetObj" @open-modal="$emit('openTagModal')" />
  <div>Add a deadline</div>
</template>
<script setup>
import Tags from "./Tags.vue";
const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});
</script>
