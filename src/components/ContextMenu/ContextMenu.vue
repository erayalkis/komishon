<template>
  <div>
    <div
      class="flex-col absolute bg-neutral-50 border-2 rounded-sm border-gray-200 z-50 outline-none w-48"
      v-if="opened"
      tabindex="-1"
      ref="menu"
      :style="{ top: top, left: left }"
      @blur="close"
    >
      <!-- <div class="px-3 text-xl text-center border-b border-gray-200">
      {{ truncateFilenameIfTooLong(targetObj.file_name) }}
    </div> -->

      <div
        class="fav-item flex cursor-pointer hover:bg-gray-100 transition duration-300 ease-out py-2 px-1"
        @click="updateFileFav"
      >
        <svg
          class="file-heart w-6 h-6 mr-2"
          :class="{
            'file-heart': !targetObj.favorited,
            'file-heart-fill': targetObj.favorited,
          }"
          @click="favoriteFile"
        >
          <use href="../../assets/Heart.svg#svgHeartEmpty"></use>
        </svg>
        <p>{{ targetObj.favorited ? "Unfavorite file" : "Favorite file" }}</p>
      </div>

      <FolderItems v-if="isFolder" :target-obj="targetObj" />
      <FileItems
        v-if="isFile"
        :target-obj="targetObj"
        :uses-props="usesProps"
      />
    </div>
  </div>
</template>
<script setup>
import { ref, nextTick } from "vue";
import FileItems from "./Items/FileItems.vue";
import FolderItems from "./Items/FolderItems.vue";
import { useStore } from "vuex";

const { dispatch } = useStore();

const props = defineProps({
  usesProps: Boolean,
});

const emit = defineEmits(["updatePropsFileFav"]);

const isFile = ref(false);
const isFolder = ref(false);
const targetObj = ref({});
const opened = ref(false);
const top = ref("0px");
const left = ref("0px");
const menu = ref(null);

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

const updateFileFav = async () => {
  dispatch("updateFileFavStatus", {
    file: targetObj.value,
    isFav: targetObj.value.favorited ? 0 : 1,
  });
  if (props.usesProps) {
    emit("updatePropsFileFav", targetObj.value.id);
  }
};

defineExpose({
  close,
  open,
});
</script>
<style scope>
.fav-item:hover .file-heart {
  fill: #b595ff;
  transition: 200ms ease-out fill;
}

.fav-item:hover .file-heart-fill {
  fill: #6036c0;
  transition: 200ms ease-out fill;
}
</style>
