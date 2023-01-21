<template>
  <ContextMenu ref="ctxMenu" />
  <div
    class="flex flex-col gap-2 px-3 py-3 overflow-y-auto"
    style="height: 96%"
  >
    <template v-for="child in children" :key="child.id">
      <FileRow
        :data="child"
        @contextmenu.prevent="$refs.ctxMenu.open($event, child)"
      />
    </template>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";
import FileRow from "./FileRow.vue";
import ContextMenu from "../ContextMenu/ContextMenu.vue";
const { state } = useStore();

const children = computed(() => state.files.children);
</script>
