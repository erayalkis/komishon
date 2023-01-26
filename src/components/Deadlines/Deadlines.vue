<template>
  <v-date-picker
    v-model="chosenDate"
    :attributes="dateObj"
    :select-attribute="selectAttribute"
    :model-config="dateConfig"
  />
</template>
<script setup>
import { onMounted, computed, ref, watch } from "vue";
import { useStore } from "vuex";
import File from "@/components/Files/File.vue";
import "v-calendar/dist/style.css";

const { dispatch, state } = useStore();

const deadlines = computed(() => state.deadlines.deadlines);
const files = ref([]);
const chosenDate = ref(new Date());

onMounted(async () => {
  await dispatch("loadDeadlines");
  const date = chosenDate.value;
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  files.value = dispatch("getFilesByDeadlineDate", chosenDate.value);
});

watch(chosenDate, () => {
  const date = new Date(chosenDate.value);
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  const files = dispatch("getFilesByDeadlineDate", date);

  console.log(files);
});

const dateObj = computed(() => {
  console.log(deadlines.value);
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
