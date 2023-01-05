<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center justify-between">
        <h1 class="text-4xl text-gray-900">Add a tag</h1>
        <img
          :src="X"
          class="mr-3 cursor-pointer"
          @click="$emit('closeTagModal')"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col">
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a colour:</h1>
          <input type="color" class="w-24 h-24" v-model="color" />
        </div>
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a name:</h1>
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
          class="w-32 h-24 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          @click="saveTag"
          class="w-32 h-24 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
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
import { ref } from "vue";

const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});

const emit = defineEmits(["closeTagModal"]);

const name = ref("");
const color = ref("");

const saveTag = () => {
  if (!name.value) return;

  const newTag = {
    tag_name: name.value,
    parent_path: props.targetObj.path,
    parent_id: props.targetObj.id,
    color: color.value,
  };

  console.log(newTag);
  emit("closeTagModal");
};
</script>
