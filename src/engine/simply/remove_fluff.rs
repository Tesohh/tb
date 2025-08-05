use std::sync::Arc;

use crate::engine::dom::{Select, SharedNode};

/// this function will try to extract the main part of a html tree.
///
/// it will first try to check the provided ruleset
///
/// then it will try to pick out the <main> section
/// lastly it will try to pick out the <body> section
/// if even that is not present, what kind of website are you even viewing?
pub fn remove_fluff(tree: SharedNode) -> super::Result<SharedNode> {
    // TODO: it will first try to check the provided ruleset

    let main = tree.query_select("main")?;
    let main = main.get(0);
    if let Some(res) = main {
        return Ok(Arc::clone(res));
    }

    let body = tree.query_select("body")?;
    let body = body.get(0);
    if let Some(res) = body {
        return Ok(Arc::clone(res));
    }

    Err(super::Error::NoMainSectionFound)
}
