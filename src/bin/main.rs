use std::collections::HashMap;

use anyhow::Result;
use tb::dom::{self, SharedNodeExt};

fn main() -> Result<()> {
    let html = dom::Node::new(dom::NodeType::Element(dom::ElementData {
        tag: "html".into(),
        attrs: HashMap::new(),
    }))
    .to_shared();

    let head = html.append_element("head", None)?;
    let body = html.append_element("body", None)?;

    Ok(())
}
