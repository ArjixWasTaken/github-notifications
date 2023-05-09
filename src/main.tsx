import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

listen("show", () => {
  window.location.reload();
})

listen("hide", () => {
  
})

listen("make_visible", () => invoke("make_visible"));

setTimeout(() => {
  invoke("make-visible")
}, 500);

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
