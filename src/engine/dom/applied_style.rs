use std::rc::Rc;

use crate::engine::stylesheet;

#[derive(Debug)]
pub struct AppliedStyle {
    key: Rc<str>,
    value: Rc<stylesheet::Value>,
    important: bool,
    origin: stylesheet::Origin,
    rule_specificity: stylesheet::Specificity,
}
