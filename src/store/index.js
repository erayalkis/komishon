import { createStore } from "vuex";
import filesystem from "./modules/filesystem/index";
import settings from "./modules/settings";

const store = createStore({
  modules: {
    files: filesystem,
    settings: settings,
  },
});

export default store;
