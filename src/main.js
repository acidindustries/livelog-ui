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
    var logs = document.getElementById("logs")
    let fragment = document.createRange().createContextualFragment(data);
    var logs_divs = document.querySelectorAll(".log");
    insert_log(id.payload, fragment, logs_divs);
    // logs.prepend(fragment);
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

function insert_log(id, toinsert, logs) {
  console.log(toinsert);
  for(var i = 0; i < logs.length; i++) {
    var current = logs[i];
    if(current.getAttribute('data-id') > id) {
      current.parentNode.insertBefore(toinsert, current);
      return;
    }
  }
  document.getElementById("logs").prepend(toinsert);
}
