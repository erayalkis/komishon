import { createStore } from "vuex";
import filesystem from "./modules/filesystem/file";

const store = createStore({
  modules: {
    files: filesystem,
  },
});

export default store;
