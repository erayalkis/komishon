import { lastEleOf } from "../../../helpers/array";
import { invoke } from "@tauri-apps/api/tauri";
import { appDataDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";

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
      let res = await invoke("get_base_dirs");

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    async fetchBaseDirs({ commit }) {
      let res = await invoke("get_base_dirs");

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    async fetchChildrenOf({ commit }, dir) {
      const basePath = dir.path;

      let res = await invoke("get_children_of", {
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
    async selectFolder({ state, dispatch }) {
      const dirSelect = await open({
        directory: true,
        multiple: false,
      });

      await invoke("walk_and_save", {
        baseDir: dirSelect,
      });

      if (state.paths.length === 1) {
        dispatch("loadInitialDirs");
      }
    },
    async returnToLastPath({ state, dispatch }) {
      const paths = state.paths;
      if (paths.length > 1) {
        const lastPath = lastEleOf(paths);
        await dispatch("fetchChildrenOf", lastPath);
        return;
      }

      await dispatch("fetchBaseDirs");
    },
    async searchByName({ commit, dispatch }, input) {
      if (input.trim() === "") {
        dispatch("returnToLastPath");
        return;
      }

      let res = await invoke("search_by_name", {
        input,
      });

      let parsed = JSON.parse(res);
      console.log(parsed);

      commit("setChildren", parsed);
    },
  },
  getters: {},
};

export default filesystem;
