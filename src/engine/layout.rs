use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use taffy::{AvailableSpace, Size};

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
    pub root: Option<taffy::NodeId>,
    map: LayoutMap,
}

impl LayoutManager {
    pub fn new() -> Self {
        Self {
            tree: taffy::TaffyTree::new(),
            root: None,
            map: LayoutMap::new(),
        }
    }

    pub fn build(&mut self, node: SharedNode) -> super::Result<()> {
        let root = self.convert_node_to_taffy(node)?;
        self.root = Some(root);
        Ok(())
    }

    fn convert_node_to_taffy(&mut self, node: SharedNode) -> super::Result<taffy::NodeId> {
        let children: Vec<taffy::NodeId> = node
            .read()
            .or(Err(shared_node::Error::Poison))?
            .children
            .iter()
            .map(|f| self.convert_node_to_taffy(f.clone()))
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

    pub fn compute(&mut self, available_space: Size<AvailableSpace>) -> super::Result<()> {
        if let Some(root) = self.root {
            Ok(self.tree.compute_layout(root, available_space)?)
        } else {
            Err(super::Error::LayoutRootNodeNone)
        }
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}
