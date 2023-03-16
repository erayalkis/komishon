<template>
  <div class="absolute top-12 right-3 bg-gray-100 border border-1px-black">
    <template v-for="notif in notifications" :key="notif.id">
      <div>
        <p>{{ notif.title }}</p>
        <p>{{ notif.body }}</p>
      </div>
    </template>
  </div>
</template>
<script setup>
import { useStore } from "vuex";
import { computed } from "vue";

const { state } = useStore();

const notifications = computed(() => state.notifications.notifications);
</script>
