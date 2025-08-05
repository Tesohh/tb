use thiserror::Error;

use crate::engine::dom;

pub mod elements;
pub mod page;
pub mod remove_fluff;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no main section found")]
    NoMainSectionFound,
    #[error("dom error: {0}")]
    DomError(#[from] dom::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
