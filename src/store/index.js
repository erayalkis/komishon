import { createStore } from "vuex";
import filesystem from "./modules/filesystem/index";

const store = createStore({
  modules: {
    files: filesystem,
  },
});

export default store;
