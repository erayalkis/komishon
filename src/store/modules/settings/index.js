import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { appDataDir } from "@tauri-apps/api/path";

// Currently only used to handle the view mode of the files.
const settings = {
  state: {
    preferredViewMode: "grid",
  },
  getters: {},
  mutations: {
    // Meant to be a function for updating all the settings, currently only updates the `preferredViewMode` property.
    updateSettings(state, newData) {
      state.preferredViewMode = newData.preferredViewMode;
    },
    // Assigns the `newStyle` parameter to the `preferredViewMode` property.
    // `newStyle` can be "grid" or "list".
    setViewStyle(state, newStyle) {
      state.preferredViewMode = newStyle;
    },
  },
  actions: {
    // Loads the userSettings.json file and assigns the returned data to the state.
    async loadSettings({ commit, dispatch }) {
      try {
        const data = await readTextFile(
          `${await appDataDir()}userSettings.json`
        );
        commit("updateSettings", JSON.parse(data));
      } catch (e) {
        console.log("caught", e);
        dispatch("saveSettings");
      }
    },
    // Saves the current state into the userSettings.json file.
    async saveSettings({ state }) {
      await writeTextFile(
        `${await appDataDir()}/userSettings.json`,
        JSON.stringify(state)
      );
    },
  },
};

export default settings;
