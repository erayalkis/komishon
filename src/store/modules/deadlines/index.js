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
    addDeadline(state, deadline) {
      state.deadlines.push(deadline);
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

      const res = await invoke("get_files_by_deadline", {
        deadline: unixStamp,
      });
      const files = JSON.parse(res);

      return files;
    },
    getUpcomingDeadlines(ctx) {
      const deadlines = ctx.state.deadlines;
      const today = new Date();
      today.setHours(3);
      today.setMinutes(0);
      today.setSeconds(0);
      const timeStamp = Math.floor(today.getTime() / 1000);

      const upcomingDeadlines = deadlines
        .filter((deadline) => deadline.date > timeStamp)
        .sort((a, b) => a.date - b.date);

      return upcomingDeadlines;
    },
    getPastDeadlines(ctx) {
      const deadlines = ctx.state.deadlines;
      const today = new Date();
      today.setHours(3);
      today.setMinutes(0);
      today.setSeconds(0);
      const timeStamp = Math.floor(today.getTime() / 1000);

      const pastDeadlines = deadlines
        .filter((deadline) => deadline.date < timeStamp)
        .sort((a, b) => a.date - b.date);

      return pastDeadlines;
    },
  },
};

export default deadlines;
