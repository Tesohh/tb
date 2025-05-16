use std::{collections::HashMap, fmt::Display};

use owo_colors::OwoColorize;
use strum_macros::Display;

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new(rules: Option<Vec<Rule>>) -> Self {
        Stylesheet {
            rules: rules.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: HashMap<String, Value>,
}

pub type Specificity = (usize, usize, usize);

#[derive(Debug)]
pub struct Selector {
    pub inner: Vec<CompoundSelector>,
    pub combinators: Vec<Combinator>,
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        self.inner
            .iter()
            .map(|s| s.specificity())
            .fold((0, 0, 0), |r, v| (r.0 + v.0, r.1 + v.1, r.2 + v.2))
    }
}

#[derive(Debug)]
pub struct CompoundSelector {
    pub id: Option<String>,
    pub tag_name: Option<String>,
    pub classes: Vec<String>,
}

impl CompoundSelector {
    pub fn specificity(&self) -> Specificity {
        let a = self.id.iter().count();
        let b = self.classes.len();
        let c = self.tag_name.iter().count();

        (a, b, c)
    }
}

#[derive(Debug)]
pub enum Combinator {
    /// Equivalent to ` ` in CSS
    Descendant,
    /// Equivalent to `>` in CSS
    Child,
    /// Equivalent to `+` in CSS
    AdjacentSibling,
    /// Equivalent to `~` in CSS
    GeneralSibling,
}

#[derive(Debug)]
pub enum Value {
    Keyword(String),
    Dimension(Dimension),
    Color(Color),
}

#[derive(Debug)]
pub struct Dimension {
    pub value: f64,
    pub unit: Unit,
}

/// Describes a CSS Unit.
/// These will eventually be transformed into "tb units"
#[derive(Debug, Display)]
pub enum Unit {
    Px,  // pixel (1/96 in)
    Pt,  // point (1/72 in)
    Q,   // quarter-millimeter 1/40 cm
    Mm,  // millimeter
    Cm,  // centimeter
    Pc,  // pica (1/6 in)
    In,  // inch
    Em,  // relative to font size of element
    Rem, // relative to font size of root
    Vh,  // relative to viewport height
    Vw,  // relative to viewport width
    #[strum(serialize = "%")]
    Percent, // relative to parent value,
    #[strum(serialize = "")]
    Unitless, // unitless. eg. `opacity`
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
                .on_truecolor(self.r, self.g, self.b)
        )
    }
}
