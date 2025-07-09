use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::dom::{shared_node, SharedNode};

type TaffyNodeContext = &'static str;
pub type Tree = taffy::TaffyTree<TaffyNodeContext>;

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

pub fn build_layout_tree(node: SharedNode) -> super::Result<(Tree, LayoutMap)> {
    let mut tree: taffy::TaffyTree<TaffyNodeContext> = taffy::TaffyTree::new();
    let mut map = LayoutMap::new();
    convert_node_to_taffy(&mut tree, &mut map, node)?;
    Ok((tree, map))
}

fn convert_node_to_taffy(
    tree: &mut taffy::TaffyTree<TaffyNodeContext>, // TEMP:
    map: &mut LayoutMap,
    node: SharedNode,
) -> super::Result<taffy::NodeId> {
    let children: Vec<taffy::NodeId> = node
        .read()
        .or(Err(shared_node::Error::Poison))?
        .children
        .iter()
        .map(|f| convert_node_to_taffy(tree, map, f.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    // TODO: add styles
    let taffy_node = tree.new_with_children(taffy::Style::DEFAULT, &children)?;
    // TODO: actually add plain text context
    tree.set_node_context(taffy_node, Some("LOREM IPSUM"))?;

    map.insert(LayoutKey(node.clone()), taffy_node);

    Ok(taffy_node)
}
