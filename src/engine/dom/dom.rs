use std::{collections::HashMap, rc::Rc, str::FromStr as _};

use pest::Parser;

use crate::engine::{
    css,
    stylesheet::{self, ComplexSelector, Specificity, Stylesheet},
};

use super::{
    iterator::NodeIterator, AppliedStyle, ElementData, GetSetAttr, Node, NodeType, Select as _,
    SharedNode,
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
        // reset styles on every Element,
        for node in NodeIterator::from(&self.root) {
            let mut w = node.write().unwrap();
            w.applied_styles.clear();
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

        // add styles from the `style` attribute
        for node in NodeIterator::from(&self.root) {
            if let Some(raw_style) = node.get_attr("style")? {
                let Some(css) =
                    css::CssParser::parse(css::Rule::declaration_list, &raw_style)?.next()
                else {
                    continue;
                };
                let prop_map = css::parse_declarations(css);
                let mut w = node.write().unwrap();
                for (k, v) in prop_map {
                    w.applied_styles.push(AppliedStyle {
                        key: Rc::clone(&k),
                        value: Rc::clone(&v),
                        origin: stylesheet::Origin::Author,
                        rule_specificity: Specificity(1, 0, 0, 0),
                    });
                }
            };
        }

        Ok(())
    }
}
