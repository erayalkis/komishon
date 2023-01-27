<template>
  <v-date-picker
    v-model="chosenDate"
    :attributes="dateObj"
    :select-attribute="selectAttribute"
    :model-config="dateConfig"
  />
  <template v-if="files.length > 0">
    <h1 class="text-xl text-gray-900">
      {{ files.length }} {{ files.length == 1 ? "deadline" : "deadlines" }} on
      {{ parsedChosenDate.toDateString() }}
    </h1>
    <Files :files="files" />
  </template>
  <template v-else>
    <h1 class="text-xl text-gray-900">
      No deadlines on {{ parsedChosenDate.toDateString() }}
    </h1>
  </template>
</template>
<script setup>
import { onMounted, computed, ref, watch } from "vue";
import { useStore } from "vuex";
import Files from "@/components/Files/Files.vue";
import "v-calendar/dist/style.css";

const { dispatch, state } = useStore();

const deadlines = computed(() => state.deadlines.deadlines);
const files = ref([]);
const chosenDate = ref(new Date());
const parsedChosenDate = computed(() => new Date(chosenDate.value));

onMounted(async () => {
  await dispatch("loadDeadlines");
  const date = chosenDate.value;
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  files.value = await dispatch("getFilesByDeadlineDate", chosenDate.value);
});

watch(chosenDate, async () => {
  const date = new Date(chosenDate.value);
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  const newFiles = await dispatch("getFilesByDeadlineDate", date);

  files.value = newFiles;
});

const dateObj = computed(() => {
  const dates = [];

  deadlines.value?.forEach((deadline) => {
    const unixStamp = deadline.date;
    const formatted = unixStamp * 1000;
    const dateObj = {
      key: deadline.title,
      dates: formatted,
      dot: {
        style: {
          "background-color": "#c084fc",
        },
      },
      popover: {
        label: deadline.title,
      },
    };

    dates.push(dateObj);
  });

  return dates;
});

const dateConfig = {
  type: "string",
  mask: "YYYY-MM-DD",
};

const selectAttribute = {
  highlight: "purple",
};
</script>
