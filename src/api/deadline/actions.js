import { invoke } from "@tauri-apps/api";

export function addDeadlineToFile(deadline) {
  if (!deadline) return;
  invoke("add_deadline_to_file", { deadline });
}
export function removeDeadlineFromFile(deadline) {
  if (!deadline) return;
  invoke("remove_deadline_from_file", { deadline });
}
