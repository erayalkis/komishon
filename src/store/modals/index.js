const deadlines = {
  state: {
    view: {
      tag: false,
      deadline: false,
    },
    targetFile: {},
  },
  getters: {},
  mutations: {
    setTagView(state, shouldView) {
      state.view.tag = shouldView;
    },
    addDeadline(state, shouldView) {
      state.view.deadline = shouldView;
    },
    setTargetFile(state, file) {
      state.targetFile = file;
    },
  },
};

export default deadlines;
