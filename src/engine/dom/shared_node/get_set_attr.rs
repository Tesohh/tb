use crate::engine::dom::NodeType;

use super::{Result, SharedNode};

pub trait GetSetAttr {
    fn set_attr(&self, key: &str, value: &str) -> Result<()>;
    fn get_attr(&self, key: &str) -> Result<Option<String>>;
}

impl GetSetAttr for SharedNode {
    fn set_attr(&self, key: &str, value: &str) -> Result<()> {
        let mut w = self.write()?;
        match &mut w.node_type {
            NodeType::Element(element_data) => {
                element_data
                    .attrs
                    .entry(String::from(key))
                    .and_modify(|v| *v = String::from(value))
                    .or_insert(String::from(value));
                Ok(())
            }
            NodeType::Text(_) => unreachable!("text nodes cannot have attributes"),
            NodeType::Comment(_) => unreachable!("comment nodes cannot have attributes"),
        }
    }

    fn get_attr(&self, key: &str) -> Result<Option<String>> {
        let r = self.read()?;
        match &r.node_type {
            NodeType::Element(element_data) => Ok(element_data.attrs.get(key).cloned()),
            NodeType::Text(_) => Ok(None),
            NodeType::Comment(_) => Ok(None),
        }
    }
}
