use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::dom::{shared_node, SharedNode};

type TaffyNodeContext = &'static str;
pub type LayoutTree = taffy::TaffyTree<TaffyNodeContext>;

pub type LayoutMap = HashMap<LayoutKey, taffy::NodeId>;
#[derive(Debug)]
pub struct LayoutKey(pub SharedNode);

impl PartialEq for LayoutKey {
    fn eq(&self, other: &Self) -> bool {
        SharedNode::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for LayoutKey {}

impl Hash for LayoutKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = Arc::as_ptr(&self.0) as *const ();
        ptr.hash(state);
    }
}

pub struct LayoutManager {
    pub tree: LayoutTree,
    map: LayoutMap,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            tree: taffy::TaffyTree::new(),
            map: LayoutMap::new(),
        }
    }

    pub fn build(&mut self, node: SharedNode) -> super::Result<taffy::NodeId> {
        let children: Vec<taffy::NodeId> = node
            .read()
            .or(Err(shared_node::Error::Poison))?
            .children
            .iter()
            .map(|f| self.build(f.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        // TODO: add styles
        let taffy_node = self
            .tree
            .new_with_children(taffy::Style::DEFAULT, &children)?;
        // TODO: actually add plain text context
        self.tree
            .set_node_context(taffy_node, Some("LOREM IPSUM"))?;

        self.map.insert(LayoutKey(node.clone()), taffy_node);

        Ok(taffy_node)
    }

    pub fn get_node_id(&self, node: SharedNode) -> Option<taffy::NodeId> {
        self.map.get(&LayoutKey(node)).copied()
    }

    pub fn get(&self, node: SharedNode) -> super::Result<&taffy::Layout> {
        let id = self
            .get_node_id(node)
            .ok_or(super::Error::LayoutNodeNotFound)?;
        Ok(self.tree.layout(id)?)
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
