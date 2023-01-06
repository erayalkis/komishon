import { invoke } from "@tauri-apps/api";

export async function addDeadlineToFile(deadline) {
  if (!deadline) return;
  const newDeadline = invoke("add_deadline_to_file", { deadline });
  return newDeadline;
}
export function removeDeadlineFromFile(deadline) {
  if (!deadline) return;
  invoke("remove_deadline_from_file", { deadline });
}
