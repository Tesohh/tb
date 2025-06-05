use crate::engine::dom::AppliedStyle;

use super::{Error, Result, SharedNode};

pub trait AskStyle {
    fn ask_style(&self, key: &str) -> Result<Option<AppliedStyle>>;
}

impl AskStyle for SharedNode {
    fn ask_style(&self, key: &str) -> Result<Option<AppliedStyle>> {
        let r = self.read()?;
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
            Ok(filtered.first().map(|s| (*s).clone()))
        } else {
            if !INHERITABLE_PROPERTIES.contains(&key) {
                return Ok(None);
            }

            let r = self.read()?;

            // if r.Parent is None, this is the root node
            let Some(ref parent) = r.parent else {
                return Ok(None);
            };

            let parent = parent.upgrade().ok_or(Error::MissingParentUpgrade)?;

            parent.ask_style(key)
        }
    }
}

static INHERITABLE_PROPERTIES: [&str; 41] = [
    "azimuth",
    "border-collapse",
    "border-spacing",
    "caption-side",
    "color",
    "cursor",
    "direction",
    "elevation",
    "empty-cells",
    "font-family",
    "font-size",
    "font-style",
    "font-variant",
    "font-weight",
    "font",
    "letter-spacing",
    "line-height",
    "list-style-image",
    "list-style-position",
    "list-style-type",
    "list-style",
    "orphans",
    "pitch-range",
    "pitch",
    "quotes",
    "richness",
    "speak-header",
    "speak-numeral",
    "speak-punctuation",
    "speak",
    "speech-rate",
    "stress",
    "text-align",
    "text-indent",
    "text-transform",
    "visibility",
    "voice-family",
    "volume",
    "white-space",
    "widows",
    "word-spacing",
];
