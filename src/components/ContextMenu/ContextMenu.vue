<template>
  <div
    class="flex-col absolute bg-gray-200 z-50 w-32 outline-none"
    v-if="opened"
    tabindex="-1"
    ref="menu"
    :style="{ top: top, left: left }"
    @blur="close"
  >
    <FolderItems v-if="isFolder" />
    <FileItems v-if="isFile" />
    <div>Test</div>
  </div>
</template>
<script setup>
import { ref, nextTick } from "vue";
import FileItems from "./Items/FileItems.vue";
import FolderItems from "./Items/FolderItems.vue";

const isFile = ref(false);
const isFolder = ref(false);
const opened = ref(false);
const top = ref("0px");
const left = ref("0px");
const menu = ref(null);

const close = () => {
  opened.value = false;
};

const open = (e) => {
  const targetsFile = e.path.some((p) => p?.classList?.contains("file"));
  const targetsFolder = e.path.some((p) => p?.classList?.contains("folder"));

  if (!targetsFile && !targetsFolder) return;

  isFile.value = targetsFile;
  isFolder.value = targetsFolder;
  opened.value = true;

  nextTick(() => {
    menu.value.focus();
    setMenu(e.y, e.x);
  });

  e.preventDefault();
};

const setMenu = (eleTop, eleLeft) => {
  const largestHeight = window.innerHeight - menu.value.offsetHeight - 25;
  const largestWidth = window.innerWidth - menu.value.offsetWidth - 25;
  if (eleTop > largestHeight) eleTop = largestHeight;
  if (eleLeft > largestWidth) eleLeft = largestWidth;
  top.value = eleTop + "px";
  left.value = eleLeft + "px";

  console.log(top.value, left.value);
};

defineExpose({
  close,
  open,
});
</script>
