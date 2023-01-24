<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center mb-6">
        <img :src="TagSvg" class="w-9 h-9 mt-1 mr-3" />
        <h1 class="text-4xl text-gray-900">Add a tag</h1>
        <img
          :src="X"
          class="ml-auto mr-3 cursor-pointer"
          @click="$emit('closeTagModal')"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col mb-6">
        <div class="flex p-3 items-center mb-6">
          <h1 class="text-3xl mr-5">Tag Colour:</h1>
          <input type="color" class="w-12 h-12 rounded-full" v-model="color" />
        </div>
        <div class="flex p-3 items-center">
          <h1 class="text-3xl mr-5">Pick a name:</h1>
          <input
            class="indent-3 text-gray-900 p-1 bg-gray-200 rounded-md"
            v-model="name"
            required
          />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-3 justify-center">
        <button
          @click="$emit('closeTagModal')"
          class="py-5 px-6 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          @click="saveTag"
          class="py-5 px-8 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
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
import { ref } from "vue";
import { addTagToFile } from "@/api/tag/actions.js";
import { useStore } from "vuex";

const { commit } = useStore();

const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});

const emit = defineEmits(["closeTagModal"]);

const name = ref("");
const color = ref("#000000");

const saveTag = async () => {
  if (!name.value) return;

  const tagData = {
    tag_name: name.value,
    parent_path: props.targetObj.path,
    parent_id: props.targetObj.id,
    color: color.value,
  };

  const tag = await addTagToFile(tagData);

  commit("addTagToFile", { id: props.targetObj.id, tag });
  emit("closeTagModal");
};
</script>
