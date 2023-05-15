import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

listen("show", () => {
  window.location.reload();
})

listen("hide", () => {
  
})

listen("make_visible", () => invoke("make_visible"));

window.addEventListener("keydown", ({ key, ctrlKey }) => {
  if (ctrlKey && ["r", "R"].includes(key)) {
    window.location.reload()
  }
}, true);

setTimeout(() => {
  invoke("make-visible")
  window.location.href = "https://github.com/notifications"
}, 500);
