use crate::engine::dom::SharedNode;

pub struct Page {
    pub content: Vec<TbElement>,
}

pub struct TbElement {
    pub kind: TbElementKind,
    pub real_node: SharedNode,
}
