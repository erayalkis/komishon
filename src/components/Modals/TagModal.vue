<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center mb-6">
        <img :src="TagSvg" class="w-9 h-9 mt-1 mr-3" />
        <h1 class="text-4xl text-gray-900">Add a tag</h1>
        <img
          :src="X"
          class="ml-auto mr-3 cursor-pointer"
          @click="closeTagModal"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col mb-6">
        <div class="flex p-3 items-center mb-6">
          <h1 class="text-3xl mr-5">Tag Colour:</h1>
          <input
            type="color"
            class="w-12 h-12 rounded-full hover:bg-gray-300 transition duration-300 ease-out"
            v-model="color"
          />
        </div>
        <div class="flex p-3 items-center">
          <h1 class="text-3xl mr-5">Tag Name:</h1>
          <input
            class="indent-3 text-gray-900 p-1 bg-gray-200 rounded-md hover:bg-gray-300 transition duration-300 ease-out"
            v-model="name"
            placeholder="Tag Name"
            required
          />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-3 justify-center">
        <button
          @click="closeTagModal"
          class="py-5 px-6 bg-violet-600 text-white hover:bg-violet-700 rounded-md transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          @click="saveTag"
          class="py-5 px-8 bg-violet-600 text-white hover:bg-violet-700 rounded-md transition duration-300 ease-out"
        >
          Save
        </button>
      </div>
    </template>
  </BaseModal>
</template>
<script setup>
import BaseModal from "./ModalBase.vue";
import X from "@/assets/X.svg";
import TagSvg from "@/assets/Tag.svg";
import { ref, computed } from "vue";
import { addTagToFile } from "@/api/tag/actions.js";
import { useStore } from "vuex";

const { commit, state } = useStore();

const name = ref("");
const color = ref("#000000");
const targetObj = computed(() => state.modals.targetFile);

const closeTagModal = () => {
  commit("setTagView", false);
};

const saveTag = async () => {
  if (!name.value) return;

  const tagData = {
    tag_name: name.value,
    parent_path: targetObj.value.path,
    parent_id: targetObj.value.id,
    color: color.value,
  };

  const tag = await addTagToFile(tagData);

  commit("addTagToFile", { id: targetObj.value.id, tag });
  closeTagModal();
};
</script>
