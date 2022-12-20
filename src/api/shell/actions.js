import { open } from "@tauri-apps/api/shell";

export async function openFileWithShell(filepath) {
  open(filepath);
}
