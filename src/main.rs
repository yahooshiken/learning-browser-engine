pub mod dom;
pub mod html_parser;

use std::fs;

fn main() {
    let filename = "index.html";
    let html = fs::read_to_string(filename).expect("Something went wrong");
    let dom = html_parser::parse(html);

    println!("{:?}", dom);
}
