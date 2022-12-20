<template>
  <div
    class="w-32 h-32 border border-black cursor-pointer rounded-md select-none"
    @dblclick="goTo(data)"
  >
    <img v-if="data.is_dir" :src="FolderSolid" class="w-12 h-12" />
    <h1>
      {{ truncateFilenameIfTooLong(data.file_name) }}
    </h1>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import FolderSolid from "@/assets/FolderSolid.svg?url";
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
