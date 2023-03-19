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
    // Sets the visibility of the tag modal.
    // Takes a boolean parameter, `shouldView`, and assigns it to the `view.tag` property.
    setTagView(state, shouldView) {
      state.view.tag = shouldView;
    },
    // Sets the visibility of the deadline modal.
    // Takes a boolean parameter, `shouldView`, and assigns it to the `view.deadline` property.
    setDeadlineView(state, shouldView) {
      state.view.deadline = shouldView;
    },
    // Sets the target file for the modals.
    // To avoid prop drilling and messy code, the target file (the file which the tag/deadline will be assigned to) is temporarily stored here.
    setTargetFile(state, file) {
      state.targetFile = file;
    },
  },
};

export default modals;
