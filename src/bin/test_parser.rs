use std::fs;

use tb::engine::dom::SharedNodeExt;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("example/helloweb/index.html")?;
    let parsed = tb::engine::html::parse_from_str(&input)?;
    parsed.root.pretty_print_tree(0)
}
