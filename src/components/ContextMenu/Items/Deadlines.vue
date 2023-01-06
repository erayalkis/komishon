<template>
  <div class="tags-header">Deadlines</div>
  <div class="absolute bg-gray-200 left-40 top-12 tags-div w-44">
    <template v-for="tag in file.deadlines">
      <div class="flex items-center">
        <div class="w-3 h-3 mr-1" :style="{ backgroundColor: tag.color }"></div>
        <p>{{ deadline.title }}</p>
        <p>{{ deadline.date }}</p>
        <img
          @click="removeTag(file.id, tag)"
          :src="X"
          class="w-4 h-4 ml-auto mr-2"
        />
      </div>
    </template>
    <p @click="$emit('openDeadlineModal')">Add a deadline +</p>
  </div>
</template>
<script setup>
import X from "@/assets/X.svg";
import { removeTagFromFile } from "@/api/tag/actions.js";
import { useStore } from "vuex";
const { commit } = useStore();

defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});

defineEmits(["openModal"]);

const removeTag = async (fileId, tag) => {
  await removeTagFromFile(tag);
  commit("removeTagFromFile", { id: fileId, tag });
};
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
