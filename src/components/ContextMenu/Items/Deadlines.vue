<template>
  <div>
    <div
      class="deadlines-header relative flex hover:bg-gray-100 transition duration-300 ease-out py-2 px-1 cursor-pointer"
    >
      <img :src="CalendarSvg" class="mr-2" />
      Deadlines
    </div>
    <div
      class="absolute bg-neutral-50 deadlines-div w-60 flex border-2 rounded-sm border-gray-200 transition duration-300 ease-out cursor-pointer"
    >
      <template v-if="file.deadlines.length == 0">
        <h1 class="p-3">No deadlines available!</h1>
      </template>
      <template v-else v-for="deadline in file.deadlines">
        <div
          class="flex items-center hover:bg-gray-100 transition duration-300 ease-out p-3"
        >
          <p class="mr-3">{{ truncatedName(deadline.title) }}:</p>
          <p>Due {{ new Date(deadline.date * 1000).toDateString() }}</p>
          <img
            @click="removeDeadline(file.id, deadline)"
            :src="X"
            class="w-4 h-4 ml-auto mr-2"
          />
        </div>
      </template>
      <p
        @click="openDeadlineModal"
        class="hover:bg-gray-100 border-t border-slate-200 transition duration-300 ease-out p-2"
      >
        Add a deadline +
      </p>
    </div>
  </div>
</template>
<script setup>
import X from "@/assets/X.svg";
import CalendarSvg from "@/assets/CalendarThin.svg";
import { removeDeadlineFromFile } from "@/api/deadline/actions.js";
import { useStore } from "vuex";
const { commit } = useStore();

defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});

const openDeadlineModal = () => {
  commit("setDeadlineView", true);
};

const removeDeadline = async (fileId, deadline) => {
  await removeDeadlineFromFile(deadline);
  commit("removeDeadlineFromFile", { id: fileId, deadline });
};

const truncatedName = (name) => {
  if (name.length > 15) return name.slice(0, 12) + "...";

  return name;
};
</script>
<style scoped>
.deadlines-div {
  display: none;
  top: 80px;
  left: 188px;
  z-index: -1;
}

.deadlines-div:hover {
  display: block;
}

.deadlines-header:hover {
  border-right-color: #fafafa;
}

.deadlines-header:hover + .deadlines-div {
  display: block;
}
</style>
