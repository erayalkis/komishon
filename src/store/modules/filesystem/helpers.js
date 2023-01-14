import { invoke } from "@tauri-apps/api";

export function isSamePath(dir1, dir2) {
  return dir1.path == dir2.path;
}

export async function fetchFileFromDb(id) {
  let intId = parseInt(id);
  const res = await invoke("fetch_single_file", { id: intId });
  const file = JSON.parse(res);

  return file;
}
