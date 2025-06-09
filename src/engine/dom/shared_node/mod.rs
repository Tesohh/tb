use super::Node;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub mod append;
pub use append::*;

pub mod get_set_attr;
pub use get_set_attr::*;

pub mod pretty_print_tree;
pub use pretty_print_tree::*;

pub mod select;
pub use select::*;

pub mod ask_style;
pub use ask_style::*;
use thiserror::Error;

pub mod iterator;

pub type SharedNode = Arc<RwLock<Node>>;
pub type WeakSharedNode = Weak<RwLock<Node>>;

pub trait SharedNodeExt: Append + GetSetAttr + PrettyPrintTree + Select + AskStyle {}
impl SharedNodeExt for SharedNode {}

#[derive(Error, Debug)]
/// These errors should never happen, but you never know...
pub enum UnreachableError {
    #[error("selector's inner simple selector list is empty")]
    SelectorHasNoSimpleSelectors,
    #[error("selector has more combinators than inner selectors")]
    SelectorHasMoreCombinatorsThanSelectors,
    #[error("this node has no parent (likely it's root), and thus {0}")]
    NoParentThus(&'static str),
    #[error("this node has no parent (likely it's root)")]
    NoParent,
    #[error("node was not found in it's parent's children")]
    NodeNotFoundInParentChildren,
    #[error("node's index was found in it's parent's children, but get returned None")]
    NodeIndexExistsButGetReturnedNone,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("lock has been poisoned")]
    Poison,
    #[error("missing parent when upgrading the weak pointer")]
    MissingParentUpgrade,
    #[error("unreachable error: {0} (congratulations on finding this. please file an issue at github.com/Tesohh/tb)")]
    Unreachable(#[from] UnreachableError),
    #[error("selector parsing error. TODO: show more information")]
    SelectorParsing,
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for Error {
    fn from(_: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        Error::Poison
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'_, T>>> for Error {
    fn from(_: PoisonError<RwLockWriteGuard<'_, T>>) -> Self {
        Error::Poison
    }
}

pub type Result<T> = core::result::Result<T, Error>;
