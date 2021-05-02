// Parse a whole CSS Stylesheet.
pub fn parse(source: String) -> Stylesheet {
  let mut parser = Parser {
    position: 0,
    input: source,
  };
  Stylesheet {
    rules: parser.parse_rules(),
  }
}

// A CSS stylesheet is a series of rules.
#[derive(Debug)]
pub struct Stylesheet {
  rules: Vec<Rule>,
}

// A rule includes one or more selectors separated by commas,
// followed by a series of declarations enclosed in braces.
#[derive(Debug)]
struct Rule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

// Specifity is one of the ways a rendering engine decades which style overrides the other in a conflict.
// See a selector's specificity here: https://www.w3.org/TR/selectors/#specificity
pub type Specificity = (usize, usize, usize);

// See CSS selectors syntax here: https://www.w3.org/TR/CSS2/selector.html#selector-syntax
// In this project, a simple selector is only implemented for simplicity.
#[derive(Debug)]
enum Selector {
  Simple(SimpleSelector),
}

impl Selector {
  pub fn specificity(&self) -> Specificity {
    let Selector::Simple(ref simple) = *self;
    let a = simple.id.iter().count();
    let b = simple.class.len();
    let c = simple.tag_name.iter().count();

    (a, b, c)
  }
}

// In this project, a simple selector can include a tag name, an ID prefixed by '#',
// any number of class names  prefixed by '.', or some combination of the above.
#[derive(Debug)]
struct SimpleSelector {
  tag_name: Option<String>,
  id: Option<String>,
  class: Vec<String>,
}

// A declaration is just a name/value pair, separated by a colon and ending with a semicolon.
#[derive(Debug)]
struct Declaration {
  name: String,
  value: Value,
}

#[derive(Debug)]
enum Value {
  Keyword(String),
  Length(f32, Unit), // f32 is an 32-bit float.
  ColorValue(Color),
}

#[derive(Debug)]
enum Unit {
  Px,
}

#[derive(Debug)]
struct Color {
  r: u8, // u8 is an 8-bit unsigned integer.
  g: u8,
  b: u8,
  a: u8,
}

struct Parser {
  position: usize,
  input: String,
}

impl Parser {
  // Read the current character without consuming it.
  fn next_char(&self) -> char {
    self.input[self.position..].chars().next().unwrap()
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

  // Parse a list of rule sets, separated by optional whitespace.
  fn parse_rules(&mut self) -> Vec<Rule> {
    let mut rules = Vec::new();
    loop {
      self.consume_whitespace();
      if self.eof() {
        break;
      }
      rules.push(self.parse_rule())
    }
    rules
  }

  // Parse a rule set: `<selectors> { <declaarations> }`
  fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: self.parse_selectors(),
      declarations: self.parse_declarations(),
    }
  }

  // Parse a comma-separated list of selectors.
  fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
      selectors.push(Selector::Simple(self.parse_simple_selector()));
      self.consume_whitespace();

      match self.next_char() {
        ',' => {
          self.consume_char();
          self.consume_whitespace();
        }
        '{' => break,
        c => panic!("Unexpected character {} in selector list", c),
      }
    }
    // Return selectors with highest specifity first, for use in matching.
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
    selectors
  }

  // Parse a list of declarations enclosed in `{ ... }`.
  fn parse_declarations(&mut self) -> Vec<Declaration> {
    assert_eq!(self.consume_char(), '{');
    let mut declarations = Vec::new();
    loop {
      self.consume_whitespace();
      if self.next_char() == '}' {
        self.consume_char();
        break;
      }
      declarations.push(self.parse_declaration());
    }
    declarations
  }

  // Parse one `<property>: <value>;` declaration.
  fn parse_declaration(&mut self) -> Declaration {
    let property_name = self.parse_identifier();
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ':');
    self.consume_whitespace();
    let value = self.parse_value();
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ';');
    Declaration {
      name: property_name,
      value: value,
    }
  }

  // Methods for parsing values.
  fn parse_value(&mut self) -> Value {
    match self.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => Value::Keyword(self.parse_identifier()),
    }
  }

  fn parse_length(&mut self) -> Value {
    Value::Length(self.parse_float(), self.parse_unit())
  }

  fn parse_float(&mut self) -> f32 {
    let s = self.consume_while(|c| match c {
      '0'..='9' | '.' => true,
      _ => false,
    });
    s.parse().unwrap()
  }

  fn parse_unit(&mut self) -> Unit {
    match &*self.parse_identifier().to_ascii_lowercase() {
      "px" => Unit::Px,
      _ => panic!("Unrecognized unit"),
    }
  }

  fn parse_color(&mut self) -> Value {
    assert_eq!(self.consume_char(), '#');
    Value::ColorValue(Color {
      r: self.parse_hex_pair(),
      g: self.parse_hex_pair(),
      b: self.parse_hex_pair(),
      a: 255,
    })
  }

  // Parse two hexadecimal digits.
  fn parse_hex_pair(&mut self) -> u8 {
    let s = &self.input[self.position..self.position + 2];
    self.position += 2;
    u8::from_str_radix(s, 16).unwrap()
  }

  // Parse a propety name or keyword.
  fn parse_identifier(&mut self) -> String {
    self.consume_while(valid_identifier_char)
  }

  // Parse one simple selector, e.g.: `type#id.class1.class2.class3`
  fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {
      tag_name: None,
      id: None,
      class: Vec::new(),
    };

    while !self.eof() {
      match self.next_char() {
        '#' => {
          self.consume_char();
          selector.id = Some(self.parse_identifier());
        }
        '.' => {
          self.consume_char();
          selector.class.push(self.parse_identifier());
        }
        '*' => {
          self.consume_char();
        }
        c if valid_identifier_char(c) => {
          selector.tag_name = Some(self.parse_identifier());
        }
        _ => break,
      }
    }
    return selector;
  }
}

fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}
