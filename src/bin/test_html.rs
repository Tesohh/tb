use std::{error::Error, fs};

use tb::engine::dom::PrettyPrintTree as _;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("samples/helloweb/index.html")?;
    let dom = tb::engine::html::parse_from_str(&input)?;
    dbg!(dom.query_select(".yellow")?.len());
    dbg!(dom.query_select("body p")?.len());
    dbg!(dom.query_select("body>p")?.len());
    dbg!(dom.query_select("body>div>p")?.len());
    dbg!(dom.query_select("h1 ~ div")?.len());
    dbg!(dom.query_select("h1 + div")?.len());
    dom.root.pretty_print_tree(0)?;

    Ok(())
}
