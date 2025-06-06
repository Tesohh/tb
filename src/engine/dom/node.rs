use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::{Arc, RwLock},
};

use crate::engine::stylesheet;

use super::{AppliedStyle, SharedNode, WeakSharedNode};

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Comment(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub parent: Option<WeakSharedNode>,
    pub children: Vec<SharedNode>,

    pub applied_styles: Vec<AppliedStyle>,
}

impl Node {
    /// creates a new, orphaned, childless Node
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            parent: None,
            children: Vec::new(),
            applied_styles: Vec::new(),
        }
    }

    /// consumes the node and moves it into a Arc<RwLock<Node>> (aka SharedNode)
    pub fn into_shared(self) -> SharedNode {
        Arc::new(RwLock::new(self))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Text(text) => write!(f, "\"{}\"", text.chars().take(24).collect::<String>()),
            NodeType::Comment(comment) => write!(
                f,
                "<!-- {} -->",
                comment.chars().take(24).collect::<String>()
            ),
            NodeType::Element(element_data) => {
                write!(f, "{}", element_data.tag)?;
                for (k, v) in &element_data.attrs {
                    write!(f, " {}={}", k, v)?;
                }
                Ok(())
            }
        }
    }
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
    pub tag: String,
    pub attrs: AttrMap,
}

impl ElementData {
    pub fn new(tag: &str, attrs: Option<AttrMap>) -> Self {
        Self {
            tag: tag.into(),
            attrs: attrs.unwrap_or_default(),
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(classes) => classes.split(" ").collect(),
            None => HashSet::new(),
        }
    }

    pub fn matches_selector(&self, selector: &stylesheet::Selector) -> bool {
        let id_ok = selector.id.is_none() || selector.id.as_ref() == self.id();
        let tag_ok = selector.tag_name.is_none() || selector.tag_name.as_ref() == Some(&self.tag);

        let my_classes = self.classes(); // small optimization
        let classes_ok = selector
            .classes
            .iter()
            .all(|class| my_classes.contains(class.as_str()));

        id_ok && tag_ok && classes_ok
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::engine::stylesheet::{ComplexSelector, Selector};

    fn selector_helper(input: &str) -> Selector {
        ComplexSelector::from_str(input).unwrap().inner[0].clone()
    }

    #[test]
    fn selector_matching() {
        let element = ElementData {
            tag: "h1".into(),
            attrs: HashMap::from([
                ("class".into(), "yellow red pink".into()),
                ("id".into(), "ooo".into()),
            ]),
        };

        let matches = [
            "*",
            "h1",
            "#ooo",
            "h1#ooo",
            ".yellow",
            "h1.yellow",
            "*.yellow",
            "*.yellow.pink",
            "*.yellow.pink.red",
            "*.yellow.pink.red.pink.pink",
        ];
        let not_matches = [
            "h1.yellow#iowjefoijweijf",
            "h2.yellow",
            "pink.red.yellow.blue",
        ];

        for x in matches {
            assert!(element.matches_selector(&selector_helper(x)));
        }
        for x in not_matches {
            assert!(!element.matches_selector(&selector_helper(x)));
        }
    }
}
