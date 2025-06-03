use crate::engine::stylesheet;

#[derive(Debug)]
pub struct AppliedStyle<'a> {
    key: &'a str,
    value: &'a stylesheet::Value,
    important: bool,
    origin: stylesheet::Origin,
    rule_specificity: stylesheet::Specificity,
}
