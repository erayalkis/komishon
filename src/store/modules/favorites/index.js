import { invoke } from "@tauri-apps/api";

// Not in use right now, will be in use when the filesystem API is refactored
const favorites = {
  state: {
    favorites: [],
  },
  getters: {},
  mutations: {
    setFavorites(state, favorites) {
      state.favorites = favorites;
    },
  },
  actions: {
    async loadFavorites({ commit }) {
      const res = await invoke("fetch_favorited_files");
      const favs = JSON.parse(res);

      commit("setFavorites", favs);
    },
  },
};

export default favorites;
