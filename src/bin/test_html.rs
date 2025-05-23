use std::fs;

use tb::engine::dom::SharedNodeExt;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("samples/helloweb/index.html")?;
    let dom = tb::engine::html::parse_from_str(&input)?;
    dbg!(dom.query_select(".yellow")?.len());
    dom.root.pretty_print_tree(0)
}
