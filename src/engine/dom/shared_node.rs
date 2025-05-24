use std::sync::{Arc, RwLock, Weak};

use anyhow::{anyhow, bail};

use crate::engine::stylesheet::{self, ComplexSelector};

use super::{AttrMap, ElementData, Node, NodeType};

pub type SharedNode = Arc<RwLock<Node>>;
pub type WeakSharedNode = Weak<RwLock<Node>>;

pub trait SharedNodeExt {
    fn append_node(&self, node: Node) -> anyhow::Result<SharedNode>;
    fn append_shared_node(&self, node: SharedNode) -> anyhow::Result<SharedNode>;
    fn append_element(&self, tag: &str, attrs: Option<AttrMap>) -> anyhow::Result<SharedNode>;
    fn append_text(&self, text: &str) -> anyhow::Result<SharedNode>;
    fn append_comment(&self, text: &str) -> anyhow::Result<SharedNode>;

    fn set_attr(&self, key: &str, value: &str) -> anyhow::Result<()>;
    fn get_attr(&self, key: &str) -> anyhow::Result<Option<String>>;

    fn pretty_print_tree(&self, depth: usize) -> anyhow::Result<()>;

    fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>>;
    fn select(&self, _selector: &stylesheet::ComplexSelector) -> anyhow::Result<Vec<SharedNode>>;
    fn select_no_recursive(
        &self,
        _selector: &stylesheet::ComplexSelector,
    ) -> anyhow::Result<Vec<SharedNode>>;
}

impl SharedNodeExt for SharedNode {
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

    fn set_attr(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut w = match self.write() {
            Ok(v) => v,
            Err(e) => bail!("{}", e),
        };
        match &mut w.node_type {
            NodeType::Element(element_data) => {
                element_data
                    .attrs
                    .entry(String::from(key))
                    .and_modify(|v| *v = String::from(value))
                    .or_insert(String::from(value));
                Ok(())
            }
            NodeType::Text(_) => unreachable!("text nodes cannot have attributes"),
            NodeType::Comment(_) => unreachable!("comment nodes cannot have attributes"),
        }
    }

    fn get_attr(&self, key: &str) -> anyhow::Result<Option<String>> {
        let r = match self.read() {
            Ok(v) => v,
            Err(e) => bail!("{}", e),
        };
        match &r.node_type {
            NodeType::Element(element_data) => Ok(element_data.attrs.get(key).cloned()),
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

    fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>> {
        self.select(&ComplexSelector::from(query)?)
    }

    fn select(&self, selector: &stylesheet::ComplexSelector) -> anyhow::Result<Vec<SharedNode>> {
        let self_lock = self.read().unwrap();
        let mut candidates = vec![];

        // TODO: for tidiness purposes, make a second trait for all select helpers
        // for example, this would be a recursive selection
        // so that it can be reused later for the descendant combinator

        // for selector in selector.inner { // TEMP:
        let simple = selector.inner[0].clone(); // TEMP:

        for child in self_lock.children.iter() {
            let child_lock = child.read().unwrap(); // unwrap on poison

            // if it isnt an element, we don't even want to match it, so completely ignore it
            if let NodeType::Element(element) = &child_lock.node_type {
                if element.matches_selector(&simple) {
                    candidates.push(Arc::clone(child));
                }

                candidates.extend(child.select(selector)?);
            }
        }
        // }

        Ok(candidates)
    }

    fn select_no_recursive(
        &self,
        _selector: &stylesheet::ComplexSelector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        Ok(vec![])
    }
}
