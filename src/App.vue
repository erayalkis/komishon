<template>
  <div class="container">
    <h1>Welcome to Komishon!</h1>
    <small
      ><em
        >This is a test build, and is intended to be used only by testers</em
      ></small
    >

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
        >rust-analyzer</a
      >
    </p>
    <router-link to="/">Go home</router-link>
    <router-link to="/files">Go to files</router-link>
    <router-link to="/dev">Go to debug page</router-link>
    <router-link to="/start">Go to start page</router-link>
    <router-view />
  </div>
</template>

<script setup>
import { appDataDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import { onBeforeMount } from "vue";
import { useStore } from "vuex";
const { dispatch } = useStore();

onBeforeMount(async () => {
  const appData = await appDataDir();
  await invoke("create_db_if_not_exists", { to: `${appData}/entries.db` });
  await dispatch("loadInitialDirs");
});
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
