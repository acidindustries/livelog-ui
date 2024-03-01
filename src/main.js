import "./htmx.min.js";
import "./tauri-plugin-htmx.js";
import { Tree, JsonView } from "./collapser.js"
window.Tree = Tree;
window.JsonView = JsonView;

const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

htmx.logAll();

listen('newlog', (id) => {
  console.log(id);
  console.log("Calling invoke");
  invoke('refresh_logs', { id: id.payload }).then((data) => {
    console.log(data);
    var logs = document.getElementById("logs")
    let fragment = document.createRange().createContextualFragment(data);
    logs.prepend(fragment);
  }).catch((error) => console.log(error)); j
});

listen('clearlogs', () => {
  htmx.trigger("#mainContent", 'clearlogs');
})

window.format_variable = function (node_id, data) {
  let tree = window.Tree.CreateTree(data);
  let jsonView = new window.JsonView(tree);
  jsonView.render(document.getElementById(node_id));
}
