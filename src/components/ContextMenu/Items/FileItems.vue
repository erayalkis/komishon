<template>
  <div>
    <Tags :file="targetObj" @open-tag-modal="$emit('openTagModal')" />
    <Deadlines
      :file="targetObj"
      @open-deadline-modal="$emit('openDeadlineModal')"
    />
  </div>
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
const emits = defineEmits(["openTagModal", "openDeadlineModal"]);
</script>
