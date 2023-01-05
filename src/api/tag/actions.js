import { invoke } from "@tauri-apps/api";

export async function addTagToFile(tag) {
  if (!tag) return;
  const new_tag = await invoke("add_tag_to_file", { tag });
  console.log(new_tag);
  return new_tag;
}

export function removeTagFromFile() {
  if (!tag) return;
  invoke("remove_tag_from_file", { tag });
}
