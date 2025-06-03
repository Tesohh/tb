use std::{str::FromStr, sync::Arc};

use anyhow::Context;

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
        self.select(&ComplexSelector::from_str(query)?)
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

            let algo = match combinator {
                stylesheet::Combinator::Descendant => SharedNode::select_simple_recursive,
                stylesheet::Combinator::Child => SharedNode::select_simple_no_recursive,
                stylesheet::Combinator::AdjacentSibling => SharedNode::select_simple_only_next,
                stylesheet::Combinator::GeneralSibling => SharedNode::select_simple_all_next,
            };

            candidates = candidates
                .into_iter()
                .map(|node| algo(&node, simple))
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect();
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
    fn select_simple_all_next(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>>;
    fn select_simple_only_next(
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
            let child_lock = child.read().unwrap();

            if let NodeType::Element(element) = &child_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(child));
                }
            }
        }
        Ok(candidates)
    }

    fn select_simple_all_next(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        let self_lock = self.read().unwrap();

        let parent = self_lock
            .parent
            .as_ref()
            .context("cannot check siblings of node with no parent (likely root)")?
            .upgrade()
            .context("parent does not exist")?;

        let parent_lock = parent.read().unwrap();
        // find my position on my parent
        let index = parent_lock
            .children
            .iter()
            .position(|child| Arc::ptr_eq(child, self))
            .context("somehow, node was not found in it's parent's children")?;

        // then start the iterator from there
        let mut candidates = vec![];
        for sibling in parent_lock.children.iter().skip(index + 1) {
            let sibling_lock = sibling.read().unwrap();
            if let NodeType::Element(element) = &sibling_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(sibling));
                }
            }
        }

        Ok(candidates)
    }

    fn select_simple_only_next(
        &self,
        simple: &stylesheet::Selector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        let self_lock = self.read().unwrap();

        let parent = self_lock
            .parent
            .as_ref()
            .context("cannot check siblings of node with no parent (likely root)")?
            .upgrade()
            .context("parent does not exist")?;

        let parent_lock = parent.read().unwrap();
        let index = parent_lock
            .children
            .iter()
            .position(|child| Arc::ptr_eq(child, self))
            .context("somehow, node was not found in it's parent's children")?;

        let sibling = parent_lock.children.get(index + 1).context(
            "somehow, node's index was found in it's parent's children, but get returned None",
        )?;

        let mut candidates = vec![];
        let sibling_lock = sibling.read().unwrap();
        if let NodeType::Element(element) = &sibling_lock.node_type {
            if element.matches_selector(simple) {
                candidates.push(Arc::clone(sibling));
            }
        }

        Ok(candidates)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_selectors() {
        let input = fs::read_to_string("samples/helloweb/index.html").unwrap();
        let dom = crate::engine::html::parse_from_str(&input).unwrap();

        let basic_class = dom.query_select(".yellow").unwrap().len();
        assert_eq!(basic_class, 1);

        let child_paragraphs = dom.query_select("body>p").unwrap().len();
        assert_eq!(child_paragraphs, 1);

        let div_child_paragraphs = dom.query_select("body>div>p").unwrap().len();
        assert_eq!(div_child_paragraphs, 2);

        let body_para_descendants = dom.query_select("body p").unwrap().len();
        assert_eq!(
            body_para_descendants,
            child_paragraphs + div_child_paragraphs
        );

        assert_eq!(dom.query_select("h1 ~ div").unwrap().len(), 2);
        assert_eq!(dom.query_select("h1 + div").unwrap().len(), 1);
    }
}
