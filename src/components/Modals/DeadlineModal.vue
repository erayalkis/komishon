<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center justify-between">
        <h1 class="text-4xl text-gray-900">Add a deadline</h1>
        <img
          :src="X"
          class="mr-3 cursor-pointer"
          @click="$emit('closeDeadlineModal')"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col">
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a name:</h1>
          <input
            class="indent-3 text-gray-900 p-1 bg-gray-200 rounded-md"
            v-model="name"
            required
          />
        </div>
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a date:</h1>
          <input type="date" v-model="date" />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-3 justify-center">
        <button
          @click="$emit('closeDeadlineModal')"
          class="w-32 h-24 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          @click="saveDeadline"
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
import { addDeadlineToFile } from "@/api/deadline/actions.js";
import { useStore } from "vuex";

const { commit } = useStore();

const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});

const emit = defineEmits(["closeDeadlineModal"]);

const name = ref("");
const date = ref("");

const saveDeadline = async () => {
  if (!name.value || !date.value) return;

  const parsedDate = new Date(date.value);
  const unixStamp = Math.floor(parsedDate.getTime() / 1000);

  const deadlineData = {
    title: name.value,
    date: unixStamp,
    parent_path: props.targetObj.path,
    parent_id: props.targetObj.id,
  };

  const deadline = await addDeadlineToFile(deadlineData);

  commit("addDeadlineToFile", {
    id: props.targetObj.id,
    deadline,
  });
  emit("closeDeadlineModal");
};
</script>
