import { createStore } from "vuex";
import modals from "./modules/modals";
import deadlines from "./modules/deadlines";
import favorites from "./modules/favorites";
import filesystem from "./modules/filesystem/index";
import settings from "./modules/settings";

const store = createStore({
  modules: {
    files: filesystem,
    deadlines: deadlines,
    favorites: favorites,
    settings: settings,
    modals: modals,
  },
});

export default store;
