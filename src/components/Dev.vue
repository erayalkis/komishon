<template>
  <div class="card">
    <button type="button" @click="selectFile()">Select Folder</button>
    <button type="button" @click="callBackend()">Call backend</button>
  </div>

  <p v-if="chosenDir">You chose: {{ chosenDir }}</p>
  <template v-for="(dir, idx) in paths">
    <a @click="goTo(dir, idx)"
      ><div>{{ dir.path }}</div></a
    >
  </template>
  <template v-for="child in children">
    <div>{{ child.file_name }}</div>
    <button v-if="child.is_dir" @click="goTo(child)">Go</button>
  </template>
</template>

<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { appDataDir } from "@tauri-apps/api/path";
import { useStore } from "vuex";
const { state, dispatch } = useStore();

const chosenDir = ref("");
// const children = ref([]);
const children = computed(() => state.files.children);
const paths = computed(() => state.files.paths);

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

async function goTo(dir, idx = null) {
  dispatch("navigateTo", { dir, idx });
}

async function callBackend() {
  const appDataPath = await appDataDir();
  let res = await invoke("get_base_dirs", {
    dbPath: `${appDataPath}/entries.db`,
  });
  let parsed = JSON.parse(res);
  console.log(parsed);
  children.value = parsed;
  // let dirChildren = await getChildren(parsed[0].path);
  // console.log(children);
  // children.value = dirChildren;
}
</script>
