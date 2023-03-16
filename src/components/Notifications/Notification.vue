<template>
  <div
    class="px-3 py-1 transition duration-300 ease-out cursor-pointer select-none hover:bg-gray-100"
  >
    <div class="flex justify-between">
      <p class="font-semibold text-lg">{{ data.title }}</p>
      <img :src="X" class="text-gray-100" @click="removeNotif" />
    </div>
    <p>{{ truncateBodyIfTooLong(data.body) }}</p>
  </div>
</template>
<script setup>
import X from "@/assets/X.svg";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  data: {
    type: Object,
    default: () => {},
  },
});

const removeNotif = () => invoke("delete_notification", { id: props.data.id });

const truncateBodyIfTooLong = (string) => {
  if (string.length > 40) {
    return string.slice(0, 37) + "...";
  }

  return string;
};
</script>
