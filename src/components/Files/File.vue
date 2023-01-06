<template>
  <div
    :class="{ folder: data.is_dir, file: !data.is_dir }"
    :component-id="data.id"
    class="w-32 h-32 flex items-center relative justify-center border border-black cursor-pointer rounded-md select-none text-center"
    @dblclick="goTo(data)"
  >
    <svg
      class="absolute file-heart w-6 h-6"
      :class="{
        'file-heart': !data.favorited,
        'file-heart-fill': data.favorited,
      }"
    >
      <use href="../../assets/Heart.svg#svgHeartEmpty"></use>
    </svg>
    <div class="flex flex-col items-center">
      <img v-if="data.is_dir" :src="FolderRegular" class="w-12 h-12" />
      <img v-else :src="FileRegular" class="w-12 h-12" />
      <h1>
        {{ truncateFilenameIfTooLong(data.file_name) }}
      </h1>
    </div>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import FolderRegular from "@/assets/FolderRegular.svg?url";
import FileRegular from "@/assets/FileRegular.svg?url";
import { openFileWithShell } from "@/api/shell/actions.js";
const { dispatch } = useStore();

defineProps({
  data: Object,
});

function goTo(dir, idx = null) {
  if (!dir.is_dir) {
    openFileWithShell(dir.path);
    return;
  }
  dispatch("navigateTo", { dir, idx });
}

function truncateFilenameIfTooLong(filename) {
  if (filename.trim().length > 15) {
    return filename.slice(0, 12) + "...";
  }
  return filename;
}
</script>
<style>
.file-heart {
  right: 7px;
  top: -10px;
  fill: #94a3b8;
  stroke: rgb(48, 44, 44);
}

.file-heart-fill {
  right: 7px;
  top: -10px;
  fill: black;
  stroke: black;
}
</style>
