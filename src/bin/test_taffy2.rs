use std::fs;

use tb::engine::{self, dom::Select};

fn main() -> Result<(), taffy::TaffyError> {
    let input = fs::read_to_string("samples/helloweb/index.html").unwrap();
    let dom = tb::engine::html::parse_from_str(&input).unwrap();

    let mut layout = engine::layout::LayoutManager::new();
    layout.build(dom.root.clone()).unwrap();
    let title = dom.query_select("#title").unwrap()[0].clone();
    let l = layout.get(title).unwrap();
    dbg!(l);
    Ok(())
}
