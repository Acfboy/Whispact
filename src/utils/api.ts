import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";

export async function testCommnication() {
  try {
    await invoke("set_disposable_msg", { msg: "Hello World!" });
  } catch (e: unknown) {
    error(String(e));
    alert(e);
  }
}