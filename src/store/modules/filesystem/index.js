import { lastEleOf } from "../../../helpers/array";
import { invoke } from "@tauri-apps/api/tauri";
import { appDataDir } from "@tauri-apps/api/path";

const filesystem = {
  state: () => ({
    // Current directory being displayed to the user
    currentDir: {},
    // Children of the current directory, can be files and folders
    children: [],
    // Array of objects representing a directory, used to keep history of files being navigated
    paths: [
      {
        file_name: "Files",
        path: "/",
      },
    ],
  }),

  mutations: {
    setChildren(state, children) {
      state.children = children;
    },
    setCurrentDir(state, dir) {
      state.currentDir = dir;
    },
    addToPaths(state, dir) {
      const paths = state.paths;
      if (dir.path == lastEleOf(paths).path) return;

      paths.push(dir);
    },
    truncatePaths(state, idx) {
      const paths = state.paths;
      const trunc = paths.slice(0, idx + 1);
      state.paths = trunc;
    },
  },
  actions: {
    async loadInitialDirs({ commit }) {
      // TODO: maybe load users last dir on the history after they close the app?
      const appDataPath = await appDataDir();

      let res = await invoke("get_base_dirs", {
        dbPath: `${appDataPath}/entries.db`,
      });

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    async fetchBaseDirs({ commit }) {
      const appDataPath = await appDataDir();

      let res = await invoke("get_base_dirs", {
        dbPath: `${appDataPath}/entries.db`,
      });

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    async fetchChildrenOf({ commit }, dir) {
      const appDataPath = await appDataDir();
      const basePath = dir.path;

      let res = await invoke("get_children_of", {
        dbPath: `${appDataPath}/entries.db`,
        path: basePath,
      });

      let parsed = JSON.parse(res);
      console.log(parsed);
      commit("setChildren", parsed);
    },
    async navigateTo({ commit, state, dispatch }, { dir, idx }) {
      if (dir.path == lastEleOf(state.paths).path) return;

      commit("setCurrentDir", dir);
      commit("addToPaths", dir);
      if (idx !== null) commit("truncatePaths", idx);
      dir.path == "/"
        ? await dispatch("loadInitialDirs")
        : await dispatch("fetchChildrenOf", dir);
    },
    async selectFolder() {
      const appDataPath = await appDataDir();

      const dirSelect = open({
        directory: true,
        multiple: false,
      });

      console.log(dirSelect);
      const testValue = await invoke("walk_and_save", {
        baseDir: dirSelect,
        to: `${appDataPath}/entries.db`,
      });
    },
  },
  getters: {},
};

export default filesystem;
