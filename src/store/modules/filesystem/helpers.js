import { invoke } from "@tauri-apps/api";

// Compares to path strings and returns a boolean value indicating whether they're equal or not.
export function isSamePath(dir1, dir2) {
  return dir1.path == dir2.path;
}

// Retrieves a single file from the database using the `id` parameter.
export async function fetchFileFromDb(id) {
  let intId = parseInt(id);
  const res = await invoke("fetch_single_file", { id: intId });
  const file = JSON.parse(res);

  return file;
}
