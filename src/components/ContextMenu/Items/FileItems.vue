<template>
  <Tags :file="targetObj" @open-tag-modal="$emit('openTagModal')" />
  <Deadlines
    :file="targetObj"
    @open-deadline-modal="$emit('openDeadlineModal')"
  />
</template>
<script setup>
import Deadlines from "./Deadlines.vue";
import Tags from "./Tags.vue";
const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});
defineEmits(["openTagModal"]);
</script>
