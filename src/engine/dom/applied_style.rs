use std::rc::Rc;

use crate::engine::stylesheet::{self, PropertyValue};

#[derive(Debug, Clone)]
pub struct AppliedStyle {
    pub key: Rc<String>,
    pub value: Rc<PropertyValue>,
    pub origin: stylesheet::Origin,
    pub rule_specificity: stylesheet::Specificity,
}
