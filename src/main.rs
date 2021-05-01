pub mod dom;
pub mod html_parser;

fn main() {
    let dom = html_parser::parse("<html></html>".to_string());
    println!("{:?}", dom);
}
