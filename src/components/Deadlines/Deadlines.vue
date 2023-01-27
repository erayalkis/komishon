<template>
  <h1 class="text-3xl px-3 py-4">Deadline Report</h1>
  <div
    class="flex justify-between bg-neutral-50 p-3 border border-gray-300 m-3 rounded-md"
  >
    <div class="flex-col text-center w-1/3">
      <h1 class="text-xl text-gray-900 mb-2">Past Deadlines</h1>
      <div>
        <template
          v-if="pastDeadlines.length > 0"
          v-for="deadline in pastDeadlines"
        >
          <div
            class="bg-white my-2 rounded-md border border-gray-300 p-1 text-gray-900 flex justify-between cursor-pointer hover:bg-gray-100 transition duration-300"
            @click="setChosenDate(deadline.date)"
          >
            <h1 class="w-1/2 text-left px-3 py-1">{{ deadline.title }}</h1>
            <div class="flex w-1/2 items-center justify-end">
              <h1 class="px-3 py-1">
                {{ new Date(deadline.date * 1000).toDateString() }}
              </h1>
              <img :src="Calendar" />
            </div>
          </div>
        </template>
        <template v-else>
          <h1>No past deadlines!</h1>
        </template>
      </div>
    </div>
    <div class="flex-col text-center w-1/3">
      <h1 class="text-xl text-gray-900 mb-2">Deadline Calendar</h1>
      <v-date-picker
        v-model="chosenDate"
        ref="calendar"
        :attributes="dateObj"
        :select-attribute="selectAttribute"
        :model-config="dateConfig"
      />
    </div>
    <div class="flex-col text-center w-1/3 h-full">
      <h1 class="text-xl text-gray-900 mb-2">Upcoming Deadlines</h1>
      <div class="overflow-auto h-96">
        <template
          v-if="upcomingDeadlines.length > 0"
          v-for="deadline in upcomingDeadlines"
        >
          <div
            class="bg-white my-2 rounded-md border border-gray-300 p-1 text-gray-900 flex justify-between cursor-pointer hover:bg-gray-100 transition duration-300"
            @click="setChosenDate(deadline.date)"
          >
            <h1 class="w-1/2 text-left px-3 py-1">{{ deadline.title }}</h1>
            <div class="flex w-1/2 items-center justify-end">
              <h1 class="px-3 py-1">
                {{ new Date(deadline.date * 1000).toDateString() }}
              </h1>
              <img :src="Calendar" />
            </div>
          </div>
        </template>
        <template v-else>
          <h1>No upcoming deadlines!</h1>
        </template>
      </div>
    </div>
  </div>
  <template v-if="files.length > 0">
    <h1 class="text-2xl text-gray-900 p-3">
      {{ files.length }} {{ files.length == 1 ? "deadline" : "deadlines" }}
      {{
        parsedChosenDate.toDateString() === todaysDate
          ? "due today"
          : `on ${parsedChosenDate.toDateString()}`
      }}
    </h1>
    <Files />
  </template>
  <template v-else>
    <h1 class="text-2xl text-gray-900 p-3">
      No deadlines on
      {{
        parsedChosenDate.toDateString() === todaysDate
          ? "due today"
          : parsedChosenDate.toDateString()
      }}
    </h1>
  </template>
</template>
<script setup>
import { onMounted, computed, ref, watch } from "vue";
import { useStore } from "vuex";
import Files from "@/components/Files/Files.vue";
import Calendar from "@/assets/Calendar.svg?url";
import "v-calendar/dist/style.css";

const { dispatch, state, commit } = useStore();

const deadlines = computed(() => state.deadlines.deadlines);
const files = computed(() => state.files.children);
const upcomingDeadlines = ref([]);
const pastDeadlines = ref([]);
const calendar = ref(null);
const todaysDate = new Date().toDateString();
const chosenDate = ref(new Date());
const parsedChosenDate = computed(() => new Date(chosenDate.value));

onMounted(async () => {
  await dispatch("loadDeadlines");
  const date = chosenDate.value;
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  const deadlineFiles = await dispatch(
    "getFilesByDeadlineDate",
    chosenDate.value
  );
  commit("setChildren", deadlineFiles);
  upcomingDeadlines.value = await dispatch("getUpcomingDeadlines");
  pastDeadlines.value = await dispatch("getPastDeadlines");
});

watch(chosenDate, async () => {
  const date = new Date(chosenDate.value);
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  const newFiles = await dispatch("getFilesByDeadlineDate", date);

  commit("setChildren", newFiles);
});

const setChosenDate = async (unixStamp) => {
  const date = new Date(unixStamp * 1000);
  date.setHours(3);
  date.setMinutes(0);
  date.setSeconds(0);

  chosenDate.value = date;
  await calendar.value.move(date);
};

const dateObj = computed(() => {
  const dates = [];

  deadlines.value?.forEach((deadline) => {
    const unixStamp = deadline.date;
    const formatted = unixStamp * 1000;
    const dateObj = {
      key: deadline.id,
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
