use std::fs;

use tb::engine::dom::SharedNodeExt;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("samples/helloweb/index.html")?;
    let parsed = tb::engine::html::parse_from_str(&input)?;
    dbg!(parsed.query_select(".yellow")?);
    parsed.root.pretty_print_tree(0)
}
