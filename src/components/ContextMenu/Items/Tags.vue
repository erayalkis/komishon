<template>
  <div>
    <div
      class="tags-header relative flex hover:bg-gray-100 transition duration-300 ease-out py-2 px-1 cursor-pointer"
    >
      <img :src="TagSvg" class="mr-2" />
      Tags
      <div class="absolute bg-gray-200 left-40 top-0 tags-div w-44">
        <template v-for="tag in file.tags">
          <div class="flex items-center">
            <div
              class="w-3 h-3 mr-1"
              :style="{ backgroundColor: tag.color }"
            ></div>
            <p>{{ tag.tag_name }}</p>
            <img
              @click="removeTag(file.id, tag)"
              :src="X"
              class="w-4 h-4 ml-auto mr-2"
            />
          </div>
        </template>
        <p @click="$emit('openTagModal')">Add a tag +</p>
      </div>
    </div>
  </div>
</template>
<script setup>
import X from "@/assets/X.svg";
import TagSvg from "@/assets/Tag.svg";
import { removeTagFromFile } from "@/api/tag/actions.js";
import { useStore } from "vuex";
const { commit } = useStore();

defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});

const emits = defineEmits(["openTagModal"]);

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

.tags-header:hover > .tags-div {
  display: block;
}
</style>
