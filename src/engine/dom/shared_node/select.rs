use std::sync::Arc;

use anyhow::{bail, Context};

use crate::engine::{
    dom::NodeType,
    stylesheet::{self, ComplexSelector},
};

use super::SharedNode;

pub trait Select {
    fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>>;
    fn select(&self, selector: &stylesheet::ComplexSelector) -> anyhow::Result<Vec<SharedNode>>;
}

impl Select for SharedNode {
    fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>> {
        self.select(&ComplexSelector::from(query)?)
    }

    fn select(&self, selector: &stylesheet::ComplexSelector) -> anyhow::Result<Vec<SharedNode>> {
        let _self_lock = self.read().unwrap();

        // first pass
        let mut selectors = selector.inner.iter();
        let simple = selectors
            .next()
            .context("selector's inner simple selector list is empty (should be unreachable)")?;
        let mut candidates = self.select_simple_recursive(simple)?;

        // go through combinators
        for combinator in &selector.combinators {
            let simple = selectors.next().context(
                "selector has more combinators than inner selectors (should be unreachable)",
            )?;
            match combinator {
                stylesheet::Combinator::Descendant => {
                    let mut new_candidates = vec![];
                    for node in candidates {
                        new_candidates.extend(node.select_simple_recursive(simple)?);
                    }
                    candidates = new_candidates;
                }
                stylesheet::Combinator::Child => {
                    let mut new_candidates = vec![];
                    for node in candidates {
                        new_candidates.extend(node.select_simple_no_recursive(simple)?);
                    }
                    candidates = new_candidates;
                }
                stylesheet::Combinator::AdjacentSibling => todo!(),
                stylesheet::Combinator::GeneralSibling => todo!(),
            }
        }

        Ok(candidates)
    }
}

trait SelectHelper {
    fn select_simple_recursive(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>>;
    fn select_simple_no_recursive(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>>;
}

impl SelectHelper for SharedNode {
    fn select_simple_recursive(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        let self_lock = self.read().unwrap();
        let mut candidates = vec![];

        for child in self_lock.children.iter() {
            let child_lock = child.read().unwrap(); // unwrap on poison

            // if it isnt an element, we don't even want to match it, so completely ignore it
            if let NodeType::Element(element) = &child_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(child));
                }

                candidates.extend(child.select_simple_recursive(simple)?);
            }
        }
        Ok(candidates)
    }

    fn select_simple_no_recursive(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        let self_lock = self.read().unwrap();
        let mut candidates = vec![];

        for child in self_lock.children.iter() {
            let child_lock = child.read().unwrap(); // unwrap on poison

            // if it isnt an element, we don't even want to match it, so completely ignore it
            if let NodeType::Element(element) = &child_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(child));
                }
            }
        }
        Ok(candidates)
    }
}
