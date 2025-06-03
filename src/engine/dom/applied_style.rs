use crate::engine::stylesheet;

pub struct AppliedStyle<'a> {
    key: &'a str,
    value: &'a stylesheet::Value,
    important: bool,
    origin: stylesheet::Origin,
    rule_specificity: stylesheet::Specificity,
}
