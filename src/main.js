import "./htmx.min.js";
import "./tauri-plugin-htmx.js";
import { Tree, JsonView } from "./collapser.js"
window.Tree = Tree;
window.JsonView = JsonView;

const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

listen('newlog', () => {
  htmx.trigger("#logs", "refresh");
});

window.format_variable = function(node_id, data) {
  console.log(data);
  console.log(node_id);
  let tree = window.Tree.CreateTree(data);
  let jsonView = new window.JsonView(tree);
  jsonView.render(document.getElementById(node_id));
}
