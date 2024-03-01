class Node {
  /**
   * @description Add more attributes to the item.
   * @param {*} item
   * @param {*} key
   * @param {Node} parent
   * */
  constructor(item, key, parent) {
    this.key = key

    /** @param {string} */
    this.type = Array.isArray(item) ? "array" : item === undefined ? "null" : typeof item

    /** @param {Number} */
    this.depth = parent ? parent.depth + 1 : 0
    this.value = item
    this.parent = parent

    /** @param {[Node]} */
    this.children = []
  }
}

class Tree {
  /**
   * @description Given the root node, it will complete the children of it.
   * @param {Node} rootNode
   */
  constructor(rootNode) {
    this.root = rootNode

    const obj = this.root.value
    if (!(obj instanceof Object)) { // Array is an Object too.
      return
    }
    Object.keys(obj).forEach(keyOrIdx => {
      const value = obj[keyOrIdx]
      const subNode = new Node(value, keyOrIdx, rootNode)
      const subTree = new Tree(subNode)
      rootNode.children.push(subTree.root)
    })
  }

  /**
   * @param {string | Object} jsonData
   * @return {Tree}
   */
  static CreateTree(jsonData) {
    jsonData = typeof jsonData === "string" ? JSON.parse(jsonData) : jsonData
    const rootNode = new Node(jsonData, "root", null)
    return new Tree(rootNode)
  }
}

class JsonView {
  static DefaultColorMap = {
    text: {
      string: "text-lime-600",
      number: "text-orange-600",
      boolean: "text-indigo-600",
      array: "text-gray-600 dark:text-white",
      object: "text-gray-600 dark:text-white",
    },
    bg: {
      object: undefined,
    }
  }

  static NewConfig() {
    return JSON.parse(JSON.stringify(JsonView.DefaultColorMap))
  }

  static SEPARATOR = " : "

  /** @type {Tree} */
  #tree

  /**
   * @param {Tree} tree
   * */
  constructor(tree) {
    this.#tree = tree
  }

  /**
   * @param {Node} node
   * @param {Object} colorMap
   */
  #render(node, colorMap = JsonView.DefaultColorMap) {
    /**
     * @param {Node} node
     * */
    const getValue = (node) => {
      const typeName = node.type
      switch (typeName) {
        case "object":
          if (node.value !== null && node.value !== undefined)
            return `object {${Object.keys(node.value).length}}`
          else
            return 'null'
        case "array":
          if (node.value !== null && node.value !== undefined)
            return `array {${Object.keys(node.value).length}}`
          else
            return 'null'
        default:
          if (node.value !== null && node.value !== undefined)
            return node.value
          else
            return 'null'
      }
    }

    const arrowIcon = ["object", "array"].includes(node.type) ? node.value !== null ? `<i class="fas fa-caret-right"></i>` : "" : "";
    const divFlag = document.createRange().createContextualFragment(`<div style="margin-left:${node.depth * 18}px">${arrowIcon}</div>`)
    const divElem = divFlag.querySelector("div")

    const textColor = colorMap.text[node.type] !== undefined ? colorMap.text[node.type] : ""
    // const bgColor = colorMap.bg[node.type] !== undefined ? `background-color:${colorMap.bg[node.type]}` : ""
    const valueStyle = (textColor).length > 0 ? textColor : ""

    const keyName = node.depth !== 0 ? node.key + JsonView.SEPARATOR : "" // depth = 0 its key is "root" which is created by the system, so ignore it.
    const spanFlag = document.createRange().createContextualFragment(
      `<span class="ms-2">${keyName}<span class="${valueStyle}">${getValue(node)}</span><span class="dark:text-gray-300 text-gray-600 text-xs pl-1 text-opacity-25">(${node.type})</span></span>`
    )

    const isCollapsible = ["object", "array"].includes(node.type)

    node.children.forEach(subNode => {
      const subElem = this.#render(subNode, colorMap)

      if (isCollapsible) {
        subElem.dataset.toggle = "none";
        subElem.querySelectorAll(`*`).forEach(e => e.style.display = subElem.dataset.toggle)
        divFlag.querySelector(`i`).addEventListener("click", (e) => {
          e.stopPropagation()
          subElem.dataset.toggle = subElem.dataset.toggle === undefined ? "none" :
            subElem.dataset.toggle === "none" ? "" : "none"

          e.target.className = subElem.dataset.toggle === "none" ? "fas fa-caret-right" : "fas fa-caret-down" // Change the icon to ▶ or ▼

          subElem.querySelectorAll(`*`).forEach(e => e.style.display = subElem.dataset.toggle)
        })
      }

      spanFlag.append(subElem)
    })
    divElem.append(spanFlag)
    return divElem
  }

  /**
   * @param {Element} targetElem
   * @param {?Object} colorMap
   */
  render(targetElem, colorMap = JsonView.DefaultColorMap) {
    targetElem.prepend(this.#render(this.#tree.root, colorMap))
  }
}

export { JsonView, Tree };
