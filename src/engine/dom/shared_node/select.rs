use std::{str::FromStr, sync::Arc};

use crate::engine::{
    dom::NodeType,
    stylesheet::{self, ComplexSelector},
};

use super::{Error, Result};
use super::{SharedNode, UnreachableError};

pub trait Select {
    fn query_select(&self, query: &str) -> Result<Vec<SharedNode>>;
    fn select(&self, selector: &stylesheet::ComplexSelector) -> Result<Vec<SharedNode>>;
}

impl Select for SharedNode {
    fn query_select(&self, query: &str) -> Result<Vec<SharedNode>> {
        self.select(&ComplexSelector::from_str(query)?)
    }

    fn select(&self, selector: &stylesheet::ComplexSelector) -> Result<Vec<SharedNode>> {
        // first pass
        let mut selectors = selector.inner.iter();
        let simple = selectors
            .next()
            .ok_or(UnreachableError::SelectorHasNoSimpleSelectors)?;
        let mut candidates = self.select_simple_recursive(simple)?;

        // go through combinators
        for combinator in &selector.combinators {
            let simple = selectors
                .next()
                .ok_or(UnreachableError::SelectorHasMoreCombinatorsThanSelectors)?;

            let algo = match combinator {
                stylesheet::Combinator::Descendant => SharedNode::select_simple_recursive,
                stylesheet::Combinator::Child => SharedNode::select_simple_no_recursive,
                stylesheet::Combinator::AdjacentSibling => SharedNode::select_simple_only_next,
                stylesheet::Combinator::GeneralSibling => SharedNode::select_simple_all_next,
            };

            candidates = candidates
                .into_iter()
                .map(|node| algo(&node, simple))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect();
        }

        Ok(candidates)
    }
}

trait SelectHelper {
    fn select_simple_recursive(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>>;
    fn select_simple_no_recursive(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>>;
    fn select_simple_all_next(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>>;
    fn select_simple_only_next(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>>;
}

impl SelectHelper for SharedNode {
    fn select_simple_recursive(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>> {
        let self_lock = self.read()?;
        let mut candidates = vec![];

        for child in self_lock.children.iter() {
            let child_lock = child.read()?;

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

    fn select_simple_no_recursive(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>> {
        let self_lock = self.read()?;
        let mut candidates = vec![];

        for child in self_lock.children.iter() {
            let child_lock = child.read()?;

            if let NodeType::Element(element) = &child_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(child));
                }
            }
        }
        Ok(candidates)
    }

    fn select_simple_all_next(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>> {
        let self_lock = self.read()?;

        let parent = self_lock
            .parent
            .as_ref()
            .ok_or(UnreachableError::NoParentThus(
                "you cannot check it for siblings",
            ))?
            .upgrade()
            .ok_or(Error::MissingParentUpgrade)?;

        let parent_lock = parent.read()?;
        // find my position on my parent
        let index = parent_lock
            .children
            .iter()
            .position(|child| Arc::ptr_eq(child, self))
            .ok_or(UnreachableError::NodeNotFoundInParentChildren)?;

        // then start the iterator from there
        let mut candidates = vec![];
        for sibling in parent_lock.children.iter().skip(index + 1) {
            let sibling_lock = sibling.read()?;
            if let NodeType::Element(element) = &sibling_lock.node_type {
                if element.matches_selector(simple) {
                    candidates.push(Arc::clone(sibling));
                }
            }
        }

        Ok(candidates)
    }

    fn select_simple_only_next(&self, simple: &stylesheet::Selector) -> Result<Vec<SharedNode>> {
        let self_lock = self.read()?;

        let parent = self_lock
            .parent
            .as_ref()
            .ok_or(UnreachableError::NoParentThus(
                "you cannot check it for siblings",
            ))?
            .upgrade()
            .ok_or(Error::MissingParentUpgrade)?;

        let parent_lock = parent.read()?;
        let index = parent_lock
            .children
            .iter()
            .position(|child| Arc::ptr_eq(child, self))
            .ok_or(UnreachableError::NodeNotFoundInParentChildren)?;

        let sibling = parent_lock
            .children
            .get(index + 1)
            .ok_or(UnreachableError::NodeIndexExistsButGetReturnedNone)?;

        let mut candidates = vec![];
        let sibling_lock = sibling.read()?;
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
    #[test]
    fn test_selectors() {
        let input = r#"
        <!DOCTYPE html>

        <head>
            <title>Hello web</title>
            <style>
                .yellow {
                    color: yellow;
                }
            </style>
        </head>

        <body>
            <h1 id="title">Hello Web</h1>
            <div class="lorem-blue">
                <p>Lorem ipsum, dolor sit amet consectetur adipisicing elit. Error, consequuntur!</p>
            </div>

            <div id="second-paragraph">
                <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi, sit.</p>
            </div>

            <p class="yellow">Lorem ipsum dolor sit amet consectetur adipisicing elit. Animi, sit.</p>
        </body>
        "#;
        let dom = crate::engine::html::parse_from_str(input).unwrap();

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
