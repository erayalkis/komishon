import { invoke } from "@tauri-apps/api";

const deadlines = {
  state: {
    deadlines: {},
  },
  getters: {},
  mutations: {
    setDeadlines(state, deadlines) {
      state.deadlines = deadlines;
    },
  },
  actions: {
    async loadDeadlines({ commit }) {
      const res = await invoke("fetch_files_with_deadlines");
      const res2 = await invoke("get_deadlines");
      console.log(JSON.parse(res2));
      const deadlines = JSON.parse(res);
      commit("setDeadlines", deadlines);
    },
  },
};

export default deadlines;
