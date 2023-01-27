<template>
  <ContextMenu ref="ctxMenu" :uses-props="files !== null" />
  <div
    class="flex flex-wrap gap-2 px-3 py-3 overflow-y-auto"
    style="height: full; max-height: 96%"
  >
    <template v-if="files !== null" v-for="child in files" :key="child.id">
      <File
        :data="child"
        :uses-props="true"
        @contextmenu.prevent="$refs.ctxMenu.open($event, child)"
        @update-props-file-fav="updateFileFavStatus"
      />
    </template>
    <template v-else v-for="child in children" :key="child.file_name">
      <File
        :data="child"
        :uses-props="false"
        @contextmenu.prevent="$refs.ctxMenu.open($event, child)"
      />
    </template>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";
import File from "./File.vue";
import ContextMenu from "../ContextMenu/ContextMenu.vue";
const { state } = useStore();

const props = defineProps({
  files: {
    type: Array,
    default: () => null,
  },
});

const children = computed(() => state.files.children);

const updateFileFavStatus = (id) => {
  const target = props.files.find((file) => file.id === id);
  target.favorited = target.favorited ? 0 : 1;
};
</script>
