const filesystem = {
  state: () => ({
    // Current directory being displayed to the user
    currentDir: {},
    // Children of the current directory, can be files and folders
    children: [],
    // Array of objects representing a directory, used to keep history of files being navigated
    paths: [
      {
        file_name: "Home",
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
      state.paths.push(dir);
    },
  },
  actions: {
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
      let res = await invoke("get_children_of", {
        dbPath: `${appDataPath}/entries.db`,
        path: basePath,
      });
      let parsed = JSON.parse(res);
      return parsed;
    },
    async navigateTo({ commit }, dir) {
      if (v.path == paths.value[paths.value.length - 1]?.path) return;
      paths.value.push(v);
      let dirChildren = await getChildren(v.path);
      console.log(dirChildren);
      children.value = dirChildren;
    },
  },
  getters: {},
};
