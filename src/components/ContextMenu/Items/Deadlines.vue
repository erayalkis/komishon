<template>
  <div>
    <div class="tags-header relative">
      Deadlines

      <div class="absolute bg-gray-200 left-40 top-0 tags-div w-64">
        <template v-for="deadline in file.deadlines">
          <div class="flex items-center">
            <p>{{ deadline.title }}</p>
            <p>{{ new Date(deadline.date * 1000).toDateString() }}</p>
            <img
              @click="removeDeadline(file.id, deadline)"
              :src="X"
              class="w-4 h-4 ml-auto mr-2"
            />
          </div>
        </template>
        <p @click="$emit('openDeadlineModal')">Add a deadline +</p>
      </div>
    </div>
  </div>
</template>
<script setup>
import X from "@/assets/X.svg";
import { removeDeadlineFromFile } from "@/api/deadline/actions.js";
import { useStore } from "vuex";
const { commit } = useStore();

defineProps({
  file: {
    type: Object,
    default: () => {},
  },
});

const emits = defineEmits("openDeadlineModal");

const removeDeadline = async (fileId, deadline) => {
  await removeDeadlineFromFile(deadline);
  commit("removeDeadlineFromFile", { id: fileId, deadline });
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
