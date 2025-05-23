use std::collections::HashMap;

use anyhow::Result;
use tb::engine::dom::{self, SharedNodeExt};

fn main() -> Result<()> {
    let html = dom::Node::new(dom::NodeType::Element(dom::ElementData {
        tag: "html".into(),
        attrs: HashMap::new(),
    }))
    .to_shared();

    let head = html.append_element("head", None)?;
    let title = head.append_element("title", None)?;
    title.append_text("MY WWBSITE")?;

    let body = html.append_element("body", None)?;

    body.append_text("lorem ipsum")?;
    body.append_comment("lorem ipsum commentum")?;

    let _ = html.pretty_print_tree(0);

    Ok(())
}
