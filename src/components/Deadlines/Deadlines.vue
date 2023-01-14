<template>
  <div class="overflow-y-auto flex-col" style="height: 96%">
    <template v-for="(val, key) in deadlines">
      <h1 class="text-3xl text-gray-900 px-1">
        {{ new Date(key * 1000).toDateString() }}
      </h1>
      <br />
      <div class="flex gap-2 p-3 py-2">
        <template v-for="file in val">
          <File :data="file" />
        </template>
      </div>
      <br />
    </template>
  </div>
</template>
<script setup>
import { onMounted, computed } from "vue";
import { useStore } from "vuex";
import File from "@/components/Files/File.vue";

const { dispatch, state } = useStore();

const deadlines = computed(() => state.deadlines.deadlines);

onMounted(async () => {
  await dispatch("loadDeadlines");
});
</script>
