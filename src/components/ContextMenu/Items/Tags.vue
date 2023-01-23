<template>
  <div>
    <div
      class="tags-header relative flex hover:bg-gray-100 transition duration-300 ease-out py-2 px-1 cursor-pointer"
    >
      <img :src="TagSvg" class="mr-2" />
      Tags
    </div>

    <div
      class="absolute bg-neutral-50 tags-div w-80 flex border-2 rounded-sm border-gray-200 border-l-0 transition duration-300 ease-out cursor-pointer"
    >
      <template v-if="file.tags.length === 0">
        <h1>No tags available!</h1>
      </template>
      <template v-for="tag in file.tags">
        <div
          class="flex items-center hover:bg-gray-100 transition duration-300 ease-out"
        >
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
      <p
        @click="$emit('openTagModal')"
        class="hover:bg-gray-100 transition duration-300 ease-out"
      >
        Add a tag +
      </p>
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
<style scoped>
.tags-div {
  display: none;
  top: 38px;
  left: 188px;
  z-index: -1;
}

.tags-div:hover {
  display: block;
}
.tags-header:hover {
  border-right-color: #fafafa;
}
.tags-header:hover + .tags-div {
  display: block;
}
</style>
