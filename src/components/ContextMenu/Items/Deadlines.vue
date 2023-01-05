<template>
  <div class="tags-header">Tags</div>
  <div class="absolute bg-gray-200 left-40 top-12 tags-div w-44">
    <template v-for="tag in file.deadlines">
      <p>{{ deadline.title }}</p>
    </template>
    <p>Add a deadline +</p>
  </div>
</template>
<script setup>
defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});
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
