use std::sync::Arc;

use crate::engine::dom::{AttrMap, ElementData, Node, NodeType};

use super::{Result, SharedNode};

pub trait Append
where
    Self: std::marker::Sized,
{
    fn append_node(&self, node: Node) -> Result<Self>;
    fn append_shared_node(&self, node: SharedNode) -> Result<Self>;
    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> Result<Self>;
    fn append_text(&self, text: &str) -> Result<Self>;
    fn append_comment(&self, text: &str) -> Result<Self>;
}

impl Append for SharedNode {
    /// append a new node to the children of this node, and set the parent on the new node
    fn append_node(&self, mut node: Node) -> Result<SharedNode> {
        let weak = Arc::downgrade(&self.clone());
        node.parent = Some(weak);

        let mut w = self.write()?;
        let shared = node.into_shared();
        w.children.push(shared.clone());
        Ok(shared)
    }

    fn append_shared_node(&self, node: SharedNode) -> Result<SharedNode> {
        let weak = Arc::downgrade(&self.clone());
        node.write()?.parent = Some(weak);

        let mut self_w = self.write()?;
        self_w.children.push(node.clone());
        Ok(node)
    }

    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> Result<SharedNode> {
        let node = Node::new(NodeType::Element(ElementData {
            tag: tag.into(),
            attrs: attrs.unwrap_or_default(),
        }));

        self.append_node(node)
    }

    fn append_text(&self, text: &str) -> Result<SharedNode> {
        let node = Node::new(NodeType::Text(text.into()));
        self.append_node(node)
    }

    fn append_comment(&self, text: &str) -> Result<SharedNode> {
        let node = Node::new(NodeType::Comment(text.into()));
        self.append_node(node)
    }
}
