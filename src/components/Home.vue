<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
    <button type="button" @click="selectFİle()">Select Folder</button>
    <button type="button" @click="callBackend()">Call backend</button>
  </div>

  <p v-if="chosenDir">You chose: {{ chosenDir }}</p>
  <p>{{ greetMsg }}</p>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { readDir, writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";

const greetMsg = ref("");
const name = ref("");
const chosenDir = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function selectFİle() {
  const dirSelect = await open({
    directory: true,
    multiple: false,
  });

  console.log(dirSelect);
  const testValue = await invoke("walk_and_save", { baseDir: dirSelect });
  console.log(testValue);

  const paths = dirSelect.split("\\");
  const chosenFile = paths[paths.length - 1];

  chosenDir.value = chosenFile;
  // const files = await readDir(dirSelect, { recursive: true });
  // console.log(files);

  // await writeTextFile("userFileData.json", JSON.stringify(files), {
  //   dir: BaseDirectory.AppData,
  // });
}

async function callBackend() {
  await invoke("read_all_from_db");
}
</script>
