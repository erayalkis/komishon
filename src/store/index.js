const { createStore } = require("vuex");
const { filesystem } = require("./modules/file");

const store = createStore({
  modules: {
    files: filesystem,
  },
});

export default store;
