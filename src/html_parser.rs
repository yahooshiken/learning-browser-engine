use super::dom;
use std::collections::HashMap;

// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
  let mut parser = Parser {
    position: 0,
    input: source,
  };
  let mut nodes = parser.parse_nodes();
  // If the document contains a root element, just return it.
  // Otherwise, create one.
  if nodes.len() == 1 {
    nodes.swap_remove(0)
  } else {
    dom::elem("html".to_string(), HashMap::new(), nodes)
  }
}

struct Parser {
  position: usize, // "usize" is an unsigned integer, similar to "size_t" in C language.
  input: String,
}

impl Parser {
  // Read the current character without consuming it.
  fn next_char(&self) -> char {
    self.input[self.position..].chars().next().unwrap()
  }
  // Do the cext characters start with the given string?
  fn starts_with(&self, s: &str) -> bool {
    self.input[self.position..].starts_with(s)
  }
  // Return true if all input is consumed.
  fn eof(&self) -> bool {
    self.position >= self.input.len()
  }
  // Return the current character, and advance self.pos to the next character.
  fn consume_char(&mut self) -> char {
    let mut iterator = self.input[self.position..].char_indices(); // returns an iterator over the "char"s of a string slice, and their positions.
    let (_, current_char) = iterator.next().unwrap();
    let (next_position, _) = iterator.next().unwrap_or((1, ' '));
    self.position += next_position;

    return current_char;
  }
  // Consume characters until `test` returns false
  // See to know usage of "where" clause: https://doc.rust-lang.org/rust-by-example/generics/where.html
  fn consume_while<F>(&mut self, test: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }

    return result;
  }

  // Consume and discard zero or more whitespace characters.
  fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  // Parse a tag or attribute name.
  fn parse_tag_name(&mut self) -> String {
    self.consume_while(|c| match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' => true,
      _ => false,
    })
  }

  // Parse a single node.
  fn parse_node(&mut self) -> dom::Node {
    match self.next_char() {
      '<' => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  // Parse a text node.
  fn parse_text(&mut self) -> dom::Node {
    dom::text(self.consume_while(|c| c != '<'))
  }

  // Parse a single element, including its open tag, contents, and closing tag.
  fn parse_element(&mut self) -> dom::Node {
    // Opening tag.
    assert!(self.consume_char() == '<');
    let tag_name = self.parse_tag_name();
    let attrs = self.parse_attributes();
    assert!(self.consume_char() == '>');

    // Contents.
    let children = self.parse_nodes();

    // Closing tag.
    assert!(self.consume_char() == '<');
    assert!(self.consume_char() == '/');
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.consume_char() == '>');

    return dom::elem(tag_name, attrs, children);
  }

  // Parse a single name="value" pair.
  fn parse_attrs(&mut self) -> (String, String) {
    let name = self.parse_tag_name();
    assert!(self.consume_char() == '=');
    let value = self.parse_attr_value();

    return (name, value);
  }

  // Parse a quoted value.
  fn parse_attr_value(&mut self) -> String {
    let quote = self.consume_char();
    assert!(quote == '"' || quote == '/');
    let value = self.consume_while(|c| c != quote);
    assert!(self.consume_char() == quote);

    return value;
  }

  // Parse a list of name="value" pairs, separated by whitespace.
  fn parse_attributes(&mut self) -> dom::AttrMap {
    let mut attributes = HashMap::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '>' {
        break;
      }
      let (name, value) = self.parse_attrs();
      attributes.insert(name, value);
    }

    return attributes;
  }

  // Parse a sequence of sibling nodes.
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes = Vec::new();
    loop {
      self.consume_whitespace();
      if self.eof() | self.starts_with("</") {
        break;
      }
      nodes.push(self.parse_node());
    }

    return nodes;
  }
}
