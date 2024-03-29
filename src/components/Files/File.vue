<template>
  <div
    :class="{ folder: data.is_dir, file: !data.is_dir }"
    :component-id="data.id"
    class="w-40 h-40 bg-stone-50 border-2 border-gray-300 flex items-center relative justify-center cursor-pointer rounded-md select-none text-center hover:shadow-lg hover:border-violet-300 transition duration-300 ease-out"
    @dblclick="goTo(data)"
  >
    <svg
      class="absolute file-heart w-6 h-6"
      :class="{
        'file-heart': !data.favorited,
        'file-heart-fill': data.favorited,
      }"
      @click="favoriteFile"
    >
      <use href="../../assets/Heart.svg#svgHeartEmpty"></use>
    </svg>
    <div class="flex flex-col items-center">
      <img v-if="data.is_dir" :src="FolderRegular" class="w-14 h-14" />
      <img v-else :src="FileRegular" class="w-14 h-14 text-violet-500" />
      <h1 class="text-xl">
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

const props = defineProps({
  data: Object,
  usesProps: Boolean,
});

const emit = defineEmits(["updatePropsFileFav"]);

function goTo(dir, idx = null) {
  if (!dir.is_dir) {
    openFileWithShell(dir.path);
    return;
  }
  dispatch("navigateTo", { dir, idx });
}

const favoriteFile = () => {
  dispatch("updateFileFavStatus", {
    file: props.data,
    isFav: props.data.favorited ? 0 : 1,
  });
  if (props.usesProps) {
    emit("updatePropsFileFav", props.data.id);
  }
};

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
  fill: #fafaf9;
  stroke: rgb(48, 44, 44);
}

.file-heart:hover {
  fill: #b595ff;
  transition: 200ms ease-out fill;
}

.file-heart-fill {
  right: 7px;
  top: -10px;
  fill: #8b5cf6;
  stroke: black;
}

.file-heart-fill:hover {
  fill: #6036c0;
  transition: 200ms ease-out fill;
}
</style>
