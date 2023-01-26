import { invoke } from "@tauri-apps/api";

const deadlines = {
  state: {
    deadlines: [],
  },
  getters: {},
  mutations: {
    setDeadlines(state, deadlines) {
      state.deadlines = deadlines;
    },
  },
  actions: {
    async loadDeadlines({ commit }) {
      const res = await invoke("get_deadlines");
      const deadlines = JSON.parse(res);
      commit("setDeadlines", deadlines);
    },
    async getFilesByDeadlineDate(ctx, deadline) {
      const unixStamp = Math.floor(deadline.getTime() / 1000);
      console.log(unixStamp);

      const res = await invoke("get_files_by_deadline", {
        deadline: unixStamp,
      });
      console.log(res);
      const files = JSON.parse(res);

      return files;
    },
  },
};

export default deadlines;
