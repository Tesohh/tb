use std::collections::HashMap;

use crate::engine::stylesheet::{self, ComplexSelector};

use super::{ElementData, Node, NodeType, SharedNode, SharedNodeExt as _};

#[derive(Debug)]
pub struct Dom {
    pub doctype: String,
    pub root: SharedNode,
}

impl Dom {
    pub fn new(doctype: &str) -> Self {
        Dom {
            doctype: String::from(doctype),
            root: Node::new(NodeType::Element(ElementData {
                tag: "root".into(),
                attrs: HashMap::new(),
            }))
            .into_shared(),
        }
    }

    pub fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>> {
        self.root.select(&ComplexSelector::from(query)?)
    }

    pub fn select(
        &self,
        selector: &stylesheet::ComplexSelector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        self.root.select(selector)
    }
}
