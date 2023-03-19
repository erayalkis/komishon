import { invoke } from "@tauri-apps/api";

const deadlines = {
  state: {
    deadlines: [],
  },
  getters: {},
  mutations: {
    // Assigns the `deadlines` parameter to `state.deadlines`
    setDeadlines(state, deadlines) {
      state.deadlines = deadlines;
    },
    // Adds a deadline object to `state.deadlines`
    addDeadline(state, deadline) {
      state.deadlines.push(deadline);
    },
  },
  actions: {
    // Invokes the get_deadlines function from the backend and assigns the returned value to `state.deadlines`.
    async loadDeadlines({ commit }) {
      const res = await invoke("get_deadlines");
      const deadlines = JSON.parse(res);
      commit("setDeadlines", deadlines);

      return deadlines;
    },
    // Invokes the get_files_by_deadline function and returns a vector of File objects.
    async getFilesByDeadlineDate(ctx, deadline) {
      const unixStamp = Math.floor(deadline.getTime() / 1000);

      const res = await invoke("get_files_by_deadline", {
        deadline: unixStamp,
      });
      const files = JSON.parse(res);

      return files;
    },
    // Returns a filtered deadlines array where deadlines with a due date before the current date are removed.
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
    // Returns a filtered deadlines array where deadlines with a due date before the current date are removed.
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
