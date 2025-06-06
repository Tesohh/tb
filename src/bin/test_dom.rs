use std::{collections::HashMap, error::Error};

use tb::engine::dom::{self, Append as _, PrettyPrintTree as _};

fn main() -> Result<(), Box<dyn Error>> {
    let html = dom::Node::new(dom::NodeType::Element(dom::ElementData {
        tag: "html".into(),
        attrs: HashMap::new(),
    }))
    .into_shared();

    let head = html.append_element("head", None)?;
    let title = head.append_element("title", None)?;
    title.append_text("MY WWBSITE")?;

    let body = html.append_element("body", None)?;

    body.append_text("lorem ipsum")?;
    body.append_comment("lorem ipsum commentum")?;

    let _ = html.pretty_print_tree(0);

    Ok(())
}
