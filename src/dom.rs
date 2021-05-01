struct Node {
  // data common to all nodes.
  children: Vec<Node>,
  // data specific to each node type.
  node_type: NodeType,
}

enum NodeType {
  // You can See all node types here: https://dom.spec.whatwg.org/#dom-node-nodetype.
  // Element and Text are only implemented in this project for simplicity.
  Element(ElementData),
  Text(String),
}

struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMAp,
}

type AttrMap = HashMap<String, String>;

// Constructor function to make it easy to create new text nodes.
fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data),
  }
}

// Constructor function to make it easy to create new element nodes.
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData{ tag_name: name, attributes: attrs}}),
  }
}
