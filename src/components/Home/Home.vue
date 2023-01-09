<template>
  <HomeNav ref="homeNav" />
  <FilesList v-if="homeNav?.viewStyle === 'list'" />
  <Files v-else />
</template>
<script setup>
import { ref } from "vue";
import Files from "../Files/Files.vue";
import FilesList from "../Files/FilesList.vue";
import HomeNav from "./HomeNav.vue";

const homeNav = ref(null);
</script>
