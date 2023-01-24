<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center mb-6">
        <img :src="Calendar" class="w-9 h-9 mt-1 mr-3" />
        <h1 class="text-4xl text-gray-900 mr-3">Add a deadline</h1>
        <img
          :src="X"
          class="mr-3 cursor-pointer ml-auto"
          @click="$emit('closeDeadlineModal')"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col">
        <div class="flex flex-wrap p-3 items-center mb-6">
          <h1 class="text-3xl mr-5">Deadline Name:</h1>
          <input
            class="indent-3 text-gray-900 p-1 bg-gray-200 rounded-md"
            v-model="name"
            placeholder="Deadline Name"
            required
          />
        </div>
        <div class="flex p-3 items-center mb-6">
          <h1 class="text-3xl mr-5">Deadline date:</h1>
          <input
            type="date"
            v-model="date"
            class="bg-gray-100 rounded-md p-3"
          />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-3 justify-center">
        <button
          @click="$emit('closeDeadlineModal')"
          class="py-5 px-6 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          @click="saveDeadline"
          class="py-5 px-6 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
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
import Calendar from "@/assets/Calendar.svg";
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
