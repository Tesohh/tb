use tb::{dom::SharedNodeExt, html};

fn main() -> anyhow::Result<()> {
    let parsed = html::parse_from_str(
        "<div><head></head><span class='bold'><span>asdasd</span></span></div>",
    )?;
    parsed.root.pretty_print_tree(0)
}
