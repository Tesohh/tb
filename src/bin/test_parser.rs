use tb::{dom::SharedNodeExt, html};

fn main() -> anyhow::Result<()> {
    html::parse_from_str("<div><span>asdasd</span></div>")?
        .root
        .pretty_print_tree(0)
}
