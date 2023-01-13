import { createStore } from "vuex";
import deadlines from "./modules/deadlines";
import filesystem from "./modules/filesystem/index";
import settings from "./modules/settings";

const store = createStore({
  modules: {
    files: filesystem,
    deadlines: deadlines,
    settings: settings,
  },
});

export default store;
