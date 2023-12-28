pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painting;
pub mod style;

pub fn main() {
    // let var = dom::text("Hello, world!".to_string());
    // println!("----------- {:?}", var);

    let node =
        html::parse("<html><body><h1>hello</h1><div><p>world</p></div></body></html>".to_string());

    println!("----------- {:?}", node);
}
