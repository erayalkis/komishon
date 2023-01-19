<template>
  <TagModal
    v-if="showTagModal"
    @close-tag-modal="closeTagModal"
    :target-obj="targetObj"
  />
  <DeadlineModal
    v-if="showDeadlineModal"
    @close-deadline-modal="closeDeadlineModal"
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
    <p @click="updateFileFav(targetObj)">
      {{ targetObj.favorited ? "Unfavorite file" : "Favorite file" }}
    </p>
    <FolderItems v-if="isFolder" :target-obj="targetObj" />

    <FileItems
      v-if="isFile"
      :target-obj="targetObj"
      @open-tag-modal="openTagModal"
      @open-deadline-modal="openDeadlineModal"
    />
  </div>
</template>
<script setup>
import { ref, nextTick, computed } from "vue";
import FileItems from "./Items/FileItems.vue";
import TagModal from "@/components/Modals/TagModal.vue";
import DeadlineModal from "@/components/Modals/DeadlineModal.vue";
import FolderItems from "./Items/FolderItems.vue";
import { useStore } from "vuex";

const { dispatch } = useStore();

const isFile = ref(false);
const isFolder = ref(false);
const targetId = ref(null);
const targetObj = ref({});
const opened = ref(false);
const top = ref("0px");
const left = ref("0px");
const menu = ref(null);
const showTagModal = ref(false);
const showDeadlineModal = ref(false);

const close = () => {
  opened.value = false;
};

const open = async (e, target) => {
  if (!target) return;
  const targetIsFolder = target.is_dir;
  const targetIsFile = !target.is_dir;
  if (!targetIsFile && !targetIsFolder) return;

  isFile.value = targetIsFile;
  isFolder.value = targetIsFolder;
  opened.value = true;
  targetObj.value = target;

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

const openTagModal = () => {
  opened.value = false;
  showTagModal.value = true;
};

const closeTagModal = () => {
  showTagModal.value = false;
};

const openDeadlineModal = () => {
  opened.value = false;
  showDeadlineModal.value = true;
};

const closeDeadlineModal = () => {
  showDeadlineModal.value = false;
};

const updateFileFav = async (targetObj) => {
  await dispatch("updateFileFavStatus", {
    file: targetObj,
    isFav: targetObj.favorited == true ? 0 : 1,
  });
};

function truncateFilenameIfTooLong(filename) {
  if (!filename?.length) return;

  if (filename.trim().length > 20) {
    return filename.slice(0, 17) + "...";
  }
  return filename;
}

defineExpose({
  close,
  open,
});
</script>
