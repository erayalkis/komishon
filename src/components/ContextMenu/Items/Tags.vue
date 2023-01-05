<template>
  <div class="tags-header">Tags</div>
  <div class="absolute bg-gray-200 left-40 top-6 tags-div w-44">
    <template v-for="tag in file.tags">
      <div class="flex">
        <div class="w-3 h-3" :style="{ backgroundColor: tag.color }"></div>
        <p>{{ tag.tag_name }}</p>
      </div>
    </template>
    <p @click="$emit('openModal')">Add a tag +</p>
  </div>
</template>
<script setup>
defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});

defineEmits(["openModal"]);
</script>
<style>
.tags-div {
  display: none;
}

.tags-div:hover {
  display: block;
}

.tags-header:hover + .tags-div {
  display: block;
}
</style>
