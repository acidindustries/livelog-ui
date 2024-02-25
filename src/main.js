import "./htmx.min.js";
import "./tauri-plugin-htmx.js";

const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

listen('newlog', () => {
  console.log('Got it');
  htmx.trigger("#logs", "refresh");
});
