use std::{collections::HashMap, str::FromStr as _};

use crate::engine::stylesheet::{self, ComplexSelector, Stylesheet};

use super::{
    iterator::NodeIterator, AppliedStyle, ElementData, Node, NodeType, Select as _, SharedNode,
};

#[derive(Debug)]
pub struct Dom {
    pub doctype: String,
    pub root: SharedNode,
    pub stylesheets: Vec<Stylesheet>,
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
            stylesheets: Vec::new(),
        }
    }

    pub fn query_select(&self, query: &str) -> anyhow::Result<Vec<SharedNode>> {
        self.root.select(&ComplexSelector::from_str(query)?)
    }

    pub fn select(
        &self,
        selector: &stylesheet::ComplexSelector,
    ) -> anyhow::Result<Vec<SharedNode>> {
        self.root.select(selector)
    }

    pub fn apply_stylesheet(&mut self, stylesheet: Stylesheet) -> anyhow::Result<()> {
        self.stylesheets.push(stylesheet);
        self.refresh_styles()
    }

    pub fn refresh_styles(&mut self) -> anyhow::Result<()> {
        // reset styles on every Element
        for node in NodeIterator::from(&self.root) {
            node.write().unwrap().applied_styles.clear();
        }

        // apply styles
        for sheet in &self.stylesheets {
            for rule in &sheet.rules {
                let nodes = self.select(&rule.selector)?;

                for node in &nodes {
                    let mut w = node.write().unwrap();
                    for (k, v) in &rule.props {
                        w.applied_styles.push(AppliedStyle {
                            key: k.clone(),
                            value: v.clone(),
                            origin: sheet.origin,
                            rule_specificity: rule.selector.specificity(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}
