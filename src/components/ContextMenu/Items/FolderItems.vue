<template>
  <div
    @click="view"
    class="flex hover:bg-gray-100 transition duration-300 ease-out py-2 px-1 cursor-pointer"
  >
    <img :src="HDDIcon" class="mr-2" />
    <h1>View in file explorer</h1>
  </div>
  <div
    @click="deleteFolder"
    class="flex hover:bg-gray-100 transition duration-300 ease-out py-2 px-1 cursor-pointer"
  >
    <img :src="EyeOff" class="mr-2" />
    <h1>Unwatch folder</h1>
  </div>
</template>
<script setup>
import { openFileWithShell } from "@/api/shell/actions.js";
import HDDIcon from "@/assets/HardDrive.svg?url";
import EyeOff from "@/assets/EyeOff.svg?url";

const props = defineProps({
  targetObj: {
    type: Object,
    default: () => {},
  },
});

const view = () => {
  openFileWithShell(props.targetObj.path);
};
</script>
