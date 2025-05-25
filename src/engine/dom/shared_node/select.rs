use std::sync::Arc;

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
        let mut candidates = vec![];

        // for selector in selector.inner { // TEMP:
        let simple = &selector.inner[0]; // TEMP:
        candidates.extend(self.select_simple_recursive(simple)?);

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
