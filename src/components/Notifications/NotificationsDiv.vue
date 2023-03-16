<template>
  <div
    class="absolute top-12 right-3 bg-neutral-50 border-2 border-gray-200 rounded-md w-80"
  >
    <template
      v-if="notifications.length"
      v-for="notif in notifications"
      :key="notif.id"
    >
      <Notification :data="notif" />
    </template>

    <div v-else class="text-2xl text-center px-3 py-1">
      <h1>No notifications yet!</h1>
    </div>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";
import Notification from "./Notification.vue";

const { state } = useStore();

const notifications = computed(() => state.notifications.notifications);
</script>
