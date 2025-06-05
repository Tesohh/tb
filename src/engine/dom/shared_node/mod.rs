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
pub enum Error {
    #[error("lock has been poisoned")]
    Poison,
    #[error("missing parent when upgrading the weak pointer")]
    MissingParentUpgrade,
    #[error("selector's inner simple selector list is empty (should be unreachable)")]
    SelectorHasNoSimpleSelectors,
    #[error("generic error: {0}")]
    Generic(#[from] anyhow::Error),
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
