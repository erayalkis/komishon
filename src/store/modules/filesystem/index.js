import { lastEleOf } from "../../../helpers/array";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

const filesystem = {
  state: {
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
  },
  getters: {
    // Tries finding a file from the current children array using the `id` parameter.
    // Returns null if no matching file is found.
    getFile: (state) => (id) => {
      return state.children.find((folder) => folder.id == id);
    },
  },
  mutations: {
    // Sets the value of the `state.children` property to the `children` parameter.
    setChildren(state, children) {
      state.children = children;
    },
    // Sets the first element of the `state.paths` array to the `newBase` parameter.
    setBasePath(state, newBase) {
      state.paths[0] = newBase;
    },
    // Sets the value of the `state.currentDir` property to the new `dir` parameter.
    setCurrentDir(state, dir) {
      state.currentDir = dir;
    },
    // Adds a new directory to the `state.paths` array.
    addToPaths(state, dir) {
      const paths = state.paths;
      if (dir.path == lastEleOf(paths).path) return;

      paths.push(dir);
    },
    // Adds a new file to the current children array.
    addFileToChildren(state, { file }) {
      if (!file) return;
      state.children.push(file);
    },
    // Adds a Tag object to the `tags` array of the target file.
    addTagToFile(state, { id, tag }) {
      const files = state.children;
      const targetFile = files.find((file) => file.id === id);
      targetFile.tags.push(tag);

      state.children = files;
    },
    // Removes a Tag object from the `tags` array of the target file.
    removeTagFromFile(state, { id, tag }) {
      const files = state.children;
      const targetFile = files.find((file) => file.id === id);
      targetFile.tags = targetFile.tags.filter(
        (fileTag) => fileTag.id !== tag.id
      );

      state.children = files;
    },
    // Adds a Deadline object to the `deadlines` array of the target file.
    addDeadlineToFile(state, { id, deadline }) {
      const files = state.children;
      const targetFile = files.find((file) => file.id === id);
      console.log(id, files, targetFile);
      targetFile.deadlines.push(deadline);

      state.children = files;
    },
    // Removes a Deadline object from the `deadlines` array of the target file.
    removeDeadlineFromFile(state, { id, deadline }) {
      const files = state.children;
      const targetFile = files.find((file) => file.id === id);
      targetFile.deadlines = targetFile.deadlines.filter(
        (fileDeadline) => fileDeadline.id !== deadline.id
      );

      state.children = files;
    },
    // Removes a file from the current children array.
    // Uses a path string to find the matching file.
    removeFile(state, { path }) {
      // TODO: Update this function to use the file id for filtering when available. If an ID isn't available, use its path.
      state.children = state.children.filter((file) => file.path !== path);
    },
    // Updates a file in the children array.
    updateFile(state, { id, path, name }) {
      let children = state.children;
      let target = state.children.find((file) => file.id === id);
      if (!target) return;

      target.path = path;
      target.file_name = name;
      state.children = children;
    },
    // Removes all paths from the `state.paths` array except the root (base) path.
    truncatePaths(state, idx) {
      const paths = state.paths;
      if (paths.length === 1) return;

      const trunc = paths.slice(0, idx + 1);
      state.paths = trunc;
    },
  },
  actions: {
    // Invokes get_base-dirs and assigns the returned array of root (nase) directories to the `state.children` property.
    async loadInitialDirs({ commit }) {
      let res = await invoke("get_base_dirs");

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    // DUPLICATE: Invokes get_base-dirs and assigns the returned array of root (nase) directories to the `state.children` property.
    async fetchBaseDirs({ commit }) {
      let res = await invoke("get_base_dirs");

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    // Invokes the get_children_of function to retrieve the children of a directory. Assigns the returned array of files to the `state.children` property.
    async fetchChildrenOf({ commit }, dir) {
      const basePath = dir.path;

      let res = await invoke("get_children_of", {
        path: basePath,
      });

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    // Updates the current root directory and fetches the children accordingly.
    // Used when navigating between routes.
    async navigateTo({ commit, state, dispatch }, { dir, idx }) {
      if (dir.path == lastEleOf(state.paths).path) return;

      commit("setCurrentDir", dir);
      commit("addToPaths", dir);
      if (idx !== null) commit("truncatePaths", idx);

      dispatch("fetchDirsAccordingToPath", dir);
    },
    // Invokes walk_and_save to register the selected dir and its children onto the database.
    async selectFolder({ state, dispatch }) {
      const dirSelect = await open({
        directory: true,
        multiple: false,
      });

      if (!dirSelect) return;

      await invoke("walk_and_save", {
        baseDir: dirSelect,
      });

      if (state.paths.length === 1) {
        dispatch("loadInitialDirs");
      }
    },
    // Function for going back to the last path on the `state.paths` array.
    async returnToLastPath({ state, dispatch }) {
      const paths = state.paths;
      if (paths.length > 1) {
        const lastPath = lastEleOf(paths);
        await dispatch("fetchChildrenOf", lastPath);
        return;
      }

      await dispatch("fetchBaseDirs");
    },
    // Invokes search_by_name and assigns the returned value to the `state.children` property.
    async searchByName({ commit, dispatch }, input) {
      if (input.trim() === "") {
        dispatch("returnToLastPath");
        return;
      }

      let res = await invoke("search_by_name", {
        input,
      });

      let parsed = JSON.parse(res);
      commit("setChildren", parsed);
    },
    // Invokes update_favorite_status to set a files `favorited` field to the opposite of its current value.
    async updateFileFavStatus({ state }, { file, isFav }) {
      await invoke("update_favorite_status", { file, isFav });
      const children = state.children;
      const targetFile = children.find((child) => child.id === file.id);
      if (!targetFile) return;

      targetFile.favorited = isFav;
      state.children = children;
    },
    // Invokes fetch_favorited_files to retrieve all favorited files.
    async fetchFavoritedFiles({ state, commit }) {
      const res = await invoke("fetch_favorited_files");
      const files = JSON.parse(res);
      commit("setChildren", files);
    },
    // Sets the base (root) path (the first element of `state.paths`) to the target route (the `to` parameter)
    setRoutePath({ commit }, to) {
      commit("truncatePaths", 0);
      commit("setBasePath", {
        file_name: to.name,
        path: to.path,
      });
    },
    // Invokes fetch_files_with_deadlines to retrieve all files with one or multiple deadline(s) attached to them, and assigns the returned data to the `state.children` property.
    async fetchDeadlinedFiles({ state, commit }) {
      const res = await invoke("fetch_files_with_deadlines");
      const files = JSON.parse(res);
      commit("setChildren", files);
    },
    // Invokes fetch_single_file to retrieve and return a single file by id.
    async fetchFileById(ctx, id) {
      id = parseInt(id);

      const res = await invoke("fetch_single_file", { id });
      const file = JSON.parse(res);

      return file[0];
    },
    // Function for custom directory loading behavior based on the current router path.
    // For "/", we fetch the base (root) dirs and assign the returned value to `state.children`.
    // For "/favorites", we fetch all favorited files, and assign the returned value to `state.children`.
    // For anything else (the `default` case), this means we're just navigating through folders, so we fetch the children of the current directory and assigned the returnedvalue to `state.children`.
    fetchDirsAccordingToPath({ dispatch }, dir) {
      switch (dir.path) {
        case "/":
          dispatch("loadInitialDirs");
          break;
        case "/favorites":
          dispatch("fetchFavoritedFiles");
          break;
        default:
          dispatch("fetchChildrenOf", dir);
      }
    },
  },
};

export default filesystem;
