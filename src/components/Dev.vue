<template>
  <div class="card">
    <button type="button" @click="selectFile()">Select Folder</button>
    <button type="button" @click="callBackend()">Call backend</button>
  </div>

  <p v-if="chosenDir">You chose: {{ chosenDir }}</p>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { appDataDir } from "@tauri-apps/api/path";

const chosenDir = ref("");

async function selectFile() {
  const appDataPath = await appDataDir();

  const dirSelect = await open({
    directory: true,
    multiple: false,
  });

  console.log(dirSelect);
  const testValue = await invoke("walk_and_save", {
    baseDir: dirSelect,
    to: `${appDataPath}/entries.db`,
  });
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
  const appDataPath = await appDataDir();
  await invoke("read_all_from_db", { dbPath: `${appDataPath}/entries.db` });
}
</script>
