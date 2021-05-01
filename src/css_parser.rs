// A CSS stylesheet is a series of rules.
struct Stylesheet {
  rules: Vec<Rule>,
}

// A rule includes one or more selectors separated by commas,
// followed by a series of declarations enclosed in braces.
struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

// See CSS selectors syntax here: https://www.w3.org/TR/CSS2/selector.html#selector-syntax
// In this project, a simple selector is only implemented for simplicity.
enum Selector {
  Simple(SimpleSelector),
}

// In this project, a simple selector can include a tag name, an ID prefixed by '#',
// any number of class names  prefixed by '.', or some combination of the above.
struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  class: Vec<String>,
}

// A declaration is just a name/value pair, separated by a colon and ending with a semicolon.
struct Declaration {
  name: String,
  value: Value,
}

enum Value {
  Keyword(String),
  Length(f32, Unit), // f32 is an 32-bit float.
  ColorValue(Color),
}

enum Unit {
  Px,
}

struct Color {
  r: u8, // u8 is an 8-bit unsigned integer.
  g: u8,
  b: u8,
  a: u8,
}
