use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock, Weak},
};

use anyhow::{anyhow, bail};

#[derive(Debug)]
pub struct Dom {
    pub doctype: String,
    pub root: SharedNode,
}

impl Dom {
    pub fn new(doctype: &str) -> Self {
        Dom {
            doctype: String::from(doctype),
            root: Node::new(NodeType::Element(ElementData {
                tag: "root".into(),
                attrs: HashMap::new(),
            }))
            .to_shared(),
        }
    }
}

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

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Text(text) => write!(f, "\"{}\"", text.chars().take(24).collect::<String>()),
            NodeType::Comment(comment) => write!(
                f,
                "<!-- {} -->",
                comment.chars().take(24).collect::<String>()
            ),
            NodeType::Element(element_data) => write!(f, "{}", element_data.tag),
        }
    }
}

pub trait SharedNodeExt {
    fn append_node(&self, node: Node) -> anyhow::Result<SharedNode>;
    fn append_shared_node(&self, node: SharedNode) -> anyhow::Result<SharedNode>;
    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> anyhow::Result<SharedNode>;
    fn append_text(&self, text: &str) -> anyhow::Result<SharedNode>;
    fn append_comment(&self, text: &str) -> anyhow::Result<SharedNode>;

    fn set_attr(&self, key: &str, value: &str);
    fn get_attr(&self, key: &str) -> Option<String>;

    fn pretty_print_tree(&self, depth: usize) -> anyhow::Result<()>;
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

    fn set_attr(&self, key: &str, value: &str) {
        let mut w = self.write().unwrap();
        match &mut w.node_type {
            NodeType::Element(element_data) => {
                element_data
                    .attrs
                    .entry(String::from(key))
                    .and_modify(|v| *v = String::from(value))
                    .or_insert(String::from(value));
            }
            NodeType::Text(_) => unreachable!("text nodes cannot have attributes"),
            NodeType::Comment(_) => unreachable!("comment nodes cannot have attributes"),
        }
    }

    fn get_attr(&self, key: &str) -> Option<String> {
        let r = self.read().unwrap();
        match &r.node_type {
            NodeType::Element(element_data) => element_data.attrs.get(key).cloned(),
            NodeType::Text(_) => unreachable!("text nodes cannot have attributes"),
            NodeType::Comment(_) => unreachable!("comment nodes cannot have attributes"),
        }
    }

    fn pretty_print_tree(&self, depth: usize) -> anyhow::Result<()> {
        let indent = (0..depth).map(|_| "   ").collect::<String>();
        let node = match self.read() {
            Ok(v) => v,
            Err(e) => bail!("{}", e),
        };

        println!("{}{}", indent, node);

        for child in node.children.iter() {
            child.pretty_print_tree(depth + 1)?;
        }

        Ok(())
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
