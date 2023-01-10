import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { appDataDir } from "@tauri-apps/api/path";

const settings = {
  state: {
    preferredViewMode: "grid",
  },
  getters: {},
  mutations: {
    updateSettings(state, newData) {
      state.preferredViewMode = newData.preferredViewMode;
    },
    setViewStyle(state, newStyle) {
      state.preferredViewMode = newStyle;
    },
  },
  actions: {
    async loadSettings({ commit, dispatch }) {
      try {
        const data = await readTextFile(
          `${await appDataDir()}/userSettings.json`
        );
        commit("updateSettings", JSON.parse(data));
      } catch (e) {
        console.log("caught", e);
        dispatch("saveSettings");
      }
    },
    async saveSettings({ state }) {
      await writeTextFile(
        `${await appDataDir()}/userSettings.json`,
        JSON.stringify(state)
      );
    },
  },
};

export default settings;
