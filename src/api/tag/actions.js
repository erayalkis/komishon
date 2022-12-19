import { invoke } from "@tauri-apps/api";

export const actions = {
  addTagToFile(tag) {
    if (!tag) return;
    invoke("add_tag_to_file", tag);
  },
  removeTagFromFile() {
    if (!tag) return;
    invoke("remove_tag_from_file", tag);
  },
};
