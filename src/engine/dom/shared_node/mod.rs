use super::Node;
use std::sync::{Arc, RwLock, Weak};

pub mod append;
pub use append::*;

pub mod get_set_attr;
pub use get_set_attr::*;

pub mod pretty_print_tree;
pub use pretty_print_tree::*;

pub mod select;
pub use select::*;

pub type SharedNode = Arc<RwLock<Node>>;
pub type WeakSharedNode = Weak<RwLock<Node>>;

pub trait SharedNodeExt: Append + GetSetAttr + PrettyPrintTree + Select {}
impl SharedNodeExt for SharedNode {}
