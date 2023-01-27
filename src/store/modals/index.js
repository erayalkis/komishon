const modals = {
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
    setDeadlineView(state, shouldView) {
      state.view.deadline = shouldView;
    },
    setTargetFile(state, file) {
      state.targetFile = file;
    },
  },
};

export default modals;
