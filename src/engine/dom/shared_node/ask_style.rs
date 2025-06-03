use crate::engine::dom::AppliedStyle;

use super::SharedNode;

pub trait AskStyle {
    fn ask_style(&self, key: &str) -> anyhow::Result<Option<AppliedStyle>>;
}

impl AskStyle for SharedNode {
    fn ask_style(&self, key: &str) -> anyhow::Result<Option<AppliedStyle>> {
        let r = self.read().unwrap();
        let mut filtered: Vec<_> = r
            .applied_styles
            .iter()
            .filter(|v| v.key.as_str() == key)
            .collect();

        if !filtered.is_empty() {
            let max_origin = filtered
                .iter()
                .map(|s| s.origin.value(s.value.important))
                .max();
            if let Some(max) = max_origin {
                filtered.retain(|s| s.origin.value(s.value.important) == max)
            }

            let max_specificity = filtered.iter().map(|s| s.rule_specificity).max();
            if let Some(max) = max_specificity {
                filtered.retain(|s| s.rule_specificity == max)
            }

            // Cloning AppliedStyle is cheap.. it only contains Rc, enum and Specificity
            return Ok(filtered.first().map(|s| (*s).clone()));
        } else {
            // TODO: ask my parent...
        }

        Ok(None)
    }
}
