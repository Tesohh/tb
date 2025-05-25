use std::sync::Arc;

use crate::engine::{
    dom::NodeType,
    stylesheet::{self, ComplexSelector},
};

use super::SharedNode;

pub trait Select {
    fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>>;
    fn select(&self, _selector: &stylesheet::ComplexSelector) -> anyhow::Result<Vec<SharedNode>>;
    fn select_no_recursive(
        &self,
        _selector: &stylesheet::ComplexSelector,
    ) -> anyhow::Result<Vec<SharedNode>>;
}

impl Select for SharedNode {
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
