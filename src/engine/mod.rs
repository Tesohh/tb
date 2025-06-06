use thiserror::Error;

pub mod css;
pub mod dom;
pub mod html;
pub mod stylesheet;

#[derive(Debug, Error)]
pub enum Error {
    #[error("SharedNode error: {0}")]
    SharedNodeError(#[from] dom::shared_node::Error),
    #[error("HTML parsing error: {0}")]
    HtmlParsingError(#[from] Box<pest::error::Error<html::Rule>>),
    #[error("CSS parsing error: {0}")]
    CssParsingError(#[from] Box<pest::error::Error<css::Rule>>),
    #[error("invalid selector")]
    InvalidSelector,
}

pub type Result<T> = core::result::Result<T, Error>;
