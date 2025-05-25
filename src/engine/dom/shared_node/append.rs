use std::sync::Arc;

use anyhow::{anyhow, bail};

use crate::engine::dom::{AttrMap, ElementData, Node, NodeType};

use super::SharedNode;

pub trait Append
where
    Self: std::marker::Sized,
{
    fn append_node(&self, node: Node) -> anyhow::Result<Self>;
    fn append_shared_node(&self, node: SharedNode) -> anyhow::Result<Self>;
    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> anyhow::Result<Self>;
    fn append_text(&self, text: &str) -> anyhow::Result<Self>;
    fn append_comment(&self, text: &str) -> anyhow::Result<Self>;
}

impl Append for SharedNode {
    /// append a new node to the children of this node, and set the parent on the new node
    fn append_node(&self, mut node: Node) -> anyhow::Result<SharedNode> {
        let weak = Arc::downgrade(&self.clone());
        node.parent = Some(weak);

        let w = self.write();
        match w {
            Ok(mut w) => {
                let shared = node.into_shared();
                w.children.push(shared.clone());
                Ok(shared)
            }
            Err(_) => Err(anyhow!("poison error while appending new node!!")),
        }
    }

    fn append_shared_node(&self, node: SharedNode) -> anyhow::Result<SharedNode> {
        let weak = Arc::downgrade(&self.clone());
        match node.write() {
            Ok(mut w) => {
                w.parent = Some(weak);
            }
            Err(_) => bail!("poison error while appending new node!!"),
        }

        let w = self.write();
        match w {
            Ok(mut w) => {
                w.children.push(node.clone());
                Ok(node)
            }
            Err(_) => Err(anyhow!("poison error while appending new node!!")),
        }
    }

    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> anyhow::Result<SharedNode> {
        let node = Node::new(NodeType::Element(ElementData {
            tag: tag.into(),
            attrs: attrs.unwrap_or_default(),
        }));

        self.append_node(node)
    }

    fn append_text(&self, text: &str) -> anyhow::Result<SharedNode> {
        let node = Node::new(NodeType::Text(text.into()));
        self.append_node(node)
    }

    fn append_comment(&self, text: &str) -> anyhow::Result<SharedNode> {
        let node = Node::new(NodeType::Comment(text.into()));
        self.append_node(node)
    }
}
