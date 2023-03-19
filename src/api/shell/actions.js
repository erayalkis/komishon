import { open } from "@tauri-apps/api/shell";

// Uses the Tauri API's `open` function to open a file or a folder using the native shell.
// It works with any file extensions which have an associated app in the $PATH.
export async function openFileWithShell(filepath) {
  open(filepath);
}
