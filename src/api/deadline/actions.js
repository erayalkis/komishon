import { invoke } from "@tauri-apps/api";

// Invokes the add_deadline_to_file function from the Rust backend.
// Takes an object as a parameter, object MUST fit the structure of the Deadline struct.
export async function addDeadlineToFile(deadline) {
  if (!deadline) return;
  const newDeadline = invoke("add_deadline_to_file", { deadline });
  return newDeadline;
}

// Invokes the remove_deadline_from_file function from the Rust backend.
// Takes an object as a parameter, object MUST fit the structure of the Deadline struct.
export function removeDeadlineFromFile(deadline) {
  if (!deadline) return;
  invoke("remove_deadline_from_file", { deadline });
}
