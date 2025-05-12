use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use anyhow::anyhow;

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Comment(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    parent: Option<WeakSharedNode>,
    children: Vec<SharedNode>,
}

pub type SharedNode = Arc<RwLock<Node>>;
pub type WeakSharedNode = Weak<RwLock<Node>>;

impl Node {
    /// creates a new, orphaned, childless Node
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            parent: None,
            children: Vec::new(),
        }
    }

    /// consumes the node and moves it into a Arc<RwLock<Node>> (aka SharedNode)
    pub fn to_shared(self) -> SharedNode {
        Arc::new(RwLock::new(self))
    }
}

pub trait SharedNodeExt {
    fn append_node(&self, node: Node) -> anyhow::Result<SharedNode>;
    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> anyhow::Result<SharedNode>;
    fn append_text(&self, text: &str) -> anyhow::Result<SharedNode>;
    fn append_comment(&self, text: &str) -> anyhow::Result<SharedNode>;
}

impl SharedNodeExt for SharedNode {
    /// append a new node to the children of this node, and set the parent on the new node
    fn append_node(&self, mut node: Node) -> anyhow::Result<SharedNode> {
        let weak = Arc::downgrade(&self.clone());
        node.parent = Some(weak);

        let w = self.write();
        match w {
            Ok(mut w) => {
                let shared = node.to_shared();
                w.children.push(shared.clone());
                Ok(shared)
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

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
    pub tag: String,
    pub attrs: AttrMap,
}

impl ElementData {
    pub fn new(tag: &str, attrs: Option<AttrMap>) -> Self {
        Self {
            tag: tag.into(),
            attrs: attrs.unwrap_or_default(),
        }
    }
}
