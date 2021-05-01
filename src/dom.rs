use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
  // data common to all nodes.
  pub children: Vec<Node>,
  // data specific to each node type.
  pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
  // You can See all node types here: https://dom.spec.whatwg.org/#dom-node-nodetype.
  // Element and Text are only implemented in this project for simplicity.
  Element(ElementData),
  Text(String),
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
}

// Constructor function to make it easy to create new text nodes.
pub fn text(data: String) -> Node {
  Node {
    children: Vec::new(),
    node_type: NodeType::Text(data),
  }
}

// Constructor function to make it easy to create new element nodes.
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    }),
  }
}
