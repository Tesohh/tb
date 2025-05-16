use pest_derive::Parser;
use tb::engine::dom::SharedNodeExt;

#[derive(Parser)]
#[grammar = "grammar/css.pest"]
struct CssParser;

fn main() -> anyhow::Result<()> {
    let parsed = tb::engine::html::parse_from_str(
        "<div><head></head><span class='bold'><span>asdasd</span></span></div>",
    )?;
    parsed.root.pretty_print_tree(0)
}
