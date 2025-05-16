use tb::{dom::SharedNodeExt, engine::html};

fn main() -> anyhow::Result<()> {
    let parsed = engine::html::parse_from_str(
        "<div><head></head><span class='bold'><span>asdasd</span></span></div>",
    )?;
    parsed.root.pretty_print_tree(0)
}
