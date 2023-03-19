import { invoke } from "@tauri-apps/api";

// Invokes the add_tag_to_file function from the Rust backend.
// Takes an object as a parameter, object MUST fit the structure of the Tag struct.
export async function addTagToFile(tag) {
  if (!tag) return;
  const new_tag = await invoke("add_tag_to_file", { tag });
  return new_tag;
}

// Invokes the  remove_tag_from_file function from the Rust backend.
// Takes an object as a parameter, object MUST fit the structure of the Tag struct.
export function removeTagFromFile(tag) {
  if (!tag) return;
  invoke("remove_tag_from_file", { tag });
}
