<template>
  <TagModal
    v-if="showTagModal"
    @close-tag-modal="closeTagModal"
    :target-obj="targetObj"
  />
  <div
    class="flex-col absolute bg-gray-200 z-50 outline-none w-40"
    v-if="opened"
    tabindex="-1"
    ref="menu"
    :style="{ top: top, left: left }"
    @blur="close"
  >
    <div>{{ truncateFilenameIfTooLong(targetObj.file_name) }}</div>
    <FolderItems v-if="isFolder" :target-obj="targetObj" />
    <FileItems
      v-if="isFile"
      :target-obj="targetObj"
      @open-tag-modal="openTagModal"
    />
  </div>
</template>
<script setup>
import { ref, nextTick, computed } from "vue";
import FileItems from "./Items/FileItems.vue";
import TagModal from "@/components/Modals/TagModal.vue";
import FolderItems from "./Items/FolderItems.vue";
import { useStore } from "vuex";

const { getters } = useStore();

const isFile = ref(false);
const isFolder = ref(false);
const targetId = ref(null);
const targetObj = computed(() => getters.getFile(targetId.value));
const opened = ref(false);
const top = ref("0px");
const left = ref("0px");
const menu = ref(null);
const showTagModal = ref(false);

const close = () => {
  opened.value = false;
};

const open = (e) => {
  const targetFile = e.path.find((p) => p?.classList?.contains("file"));
  const targetFolder = e.path.find((p) => p?.classList?.contains("folder"));

  if (!targetFile && !targetFolder) return;

  const targetObj = targetFile || targetFolder;
  isFile.value = targetFile != null || targetFile != undefined;
  isFolder.value = targetFolder != null || targetFolder != undefined;
  opened.value = true;

  targetId.value = targetObj.getAttribute("component-id");
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
};

defineExpose({
  close,
  open,
});

const openTagModal = () => {
  opened.value = false;
  showTagModal.value = true;
};

const closeTagModal = () => {
  showTagModal.value = false;
};

function truncateFilenameIfTooLong(filename) {
  if (filename.trim().length > 20) {
    return filename.slice(0, 17) + "...";
  }
  return filename;
}
</script>
