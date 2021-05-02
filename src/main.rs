pub mod css_parser;
pub mod dom;
pub mod html_parser;

use std::fs;

fn main() {
    let filename = "example.css";
    let css = fs::read_to_string(filename).expect("Something went wrong");
    let res = css_parser::parse(css);

    println!("{:?}", res);
}
