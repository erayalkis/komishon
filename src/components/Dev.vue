<template>
  <div class="card">
    <button type="button" @click="selectFile()">Select Folder</button>
    <button type="button" @click="callBackend()">Call backend</button>
  </div>

  <p v-if="chosenDir">You chose: {{ chosenDir }}</p>
  <div style="display: flex; margin-right: auto; margin-left: auto">
    <template v-for="(dir, idx) in paths">
      <a @click="goTo(dir, idx)"
        ><div>{{ dir.path }}/</div></a
      >
    </template>
  </div>
  <template v-for="child in children">
    <div style="text-align: center">
      <div>
        <div>{{ child.id }} | {{ child.file_name }}</div>
        <div v-if="!child.is_dir">
          <a @click="addTag(child)">Add tag</a>
          <a @click="addDeadline(child)">Add deadline</a>
        </div>
      </div>
      <button v-if="child.is_dir" @click="goTo(child)" style="width: 50%">
        Go
      </button>
    </div>
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
}

async function goTo(dir, idx = null) {
  dispatch("navigateTo", { dir, idx });
}

async function addDeadline(dir, deadlineData) {
  const appDataPath = await appDataDir();
  deadlineData = {
    title: "test deadline",
    date: Date.now(),
    parent_id: dir.id,
    parent_path: dir.path,
  };
  // console.log(tagData);
  invoke("add_deadline_to_file", {
    dbPath: `${appDataPath}/entries.db`,
    deadline: deadlineData,
  });
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
