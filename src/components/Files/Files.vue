<template>
  <ContextMenu ref="ctxMenu" />
  <div
    class="flex flex-wrap gap-2 px-3 py-3 overflow-y-auto"
    style="height: full"
  >
    <template v-if="files !== null" v-for="child in files" :key="child.id">
      <File
        :data="child"
        @contextmenu.prevent="$refs.ctxMenu.open($event, child)"
      />
    </template>
    <template v-else v-for="child in children" :key="child.file_name">
      <File
        :data="child"
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

defineProps({
  files: {
    type: Array,
    default: () => null,
  },
});

const children = computed(() => state.files.children);
</script>
