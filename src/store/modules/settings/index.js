import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { appDataDir } from "@tauri-apps/api/path";

const defaults = {
  preferredViewMode: "grid",
};

const settings = {
  state: {
    preferredViewMode: null,
  },
  getters: {},
  mutations: {
    updateSettings({ state }, newState) {
      state = newState;
    },
  },
  actions: {
    loadSettings({ commit, dispatch }) {
      try {
        const data = readTextFile(`${appDataDir}/settings/userSettings.json`);
        commit("updateSettings", data);
      } catch (e) {
        commit("updateSettings", defaults);
        dispatch("saveSettings");
      }
    },
    saveSettings({ state }) {
      writeTextFile(`${appDataDir}/settings/userSettings.json`, state);
    },
  },
};

export default settings;
