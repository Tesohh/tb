use std::{collections::HashMap, fmt::Display, rc::Rc, str::FromStr};

use pest::Parser as _;
use strum_macros::Display;

use super::{
    css::{self},
    Error, Result,
};

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
    pub origin: Origin,
}

impl Stylesheet {
    pub fn new(rules: Option<Vec<Rule>>, origin: Origin) -> Self {
        Stylesheet {
            rules: rules.unwrap_or_default(),
            origin,
        }
    }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Describes the origin of a stylesheet.
/// Variants have this logical order: Author > User > Agent
/// You may also get the "value" manually by using `.. as u8`
pub enum Origin {
    Agent,
    User,
    Author,
}

impl Origin {
    pub fn value(&self, important: bool) -> u8 {
        match important {
            true => 5 - *self as u8,
            false => *self as u8,
        }
    }
}

pub type PropMap = HashMap<Rc<String>, Rc<PropertyValue>>;

#[derive(Debug)]
pub struct PropertyValue {
    pub value: Value,
    pub important: bool,
}

#[derive(Debug)]
pub struct Rule {
    pub selector: ComplexSelector,
    pub props: PropMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// `(a, b, c, d)`
/// `a` = 1 if the styles are defined inline
/// `b` = 1 if id is present
/// `c` = amount of classes
/// `d` = 1 if tag is present
pub struct Specificity(pub usize, pub usize, pub usize, pub usize);

#[derive(Debug)]
pub struct ComplexSelector {
    pub inner: Vec<Selector>,
    pub combinators: Vec<Combinator>,
}

impl FromStr for ComplexSelector {
    type Err = super::Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut pairs =
            css::CssParser::parse(css::Rule::complex_selector, input).map_err(|e| Box::new(e))?;
        let pair = match pairs.next() {
            Some(v) => v,
            None => return Err(Error::InvalidSelector),
        };
        Ok(css::parse_selector(pair))
    }
}

impl ComplexSelector {
    pub fn specificity(&self) -> Specificity {
        self.inner
            .iter()
            .map(|s| s.specificity())
            .fold(Specificity(0, 0, 0, 0), |r, v| {
                Specificity(0, r.0 + v.0, r.1 + v.1, r.2 + v.2)
            })
    }
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub id: Option<String>,
    pub tag_name: Option<String>,
    pub classes: Vec<String>,
}

impl Selector {
    /// TODO: add the inline (a) somehow
    pub fn specificity(&self) -> Specificity {
        let b = self.id.iter().count();
        let c = self.classes.len();
        let d = self.tag_name.iter().count();

        Specificity(0, b, c, d)
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
    Tb,  // tb unit == 1 cell (not standard CSS)
    #[strum(serialize = "%")]
    Percent, // relative to parent value,
    #[strum(serialize = "")]
    Unitless, // unitless. eg. `opacity`
    Invalid,
}

impl FromStr for Unit {
    type Err = super::Error;

    fn from_str(value: &str) -> Result<Self> {
        let value = value.to_lowercase();
        Ok(match value.as_str() {
            "px" => Unit::Px,
            "pt" => Unit::Pt,
            "q" => Unit::Q,
            "mm" => Unit::Mm,
            "pc" => Unit::Pc,
            "in" => Unit::In,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "vh" => Unit::Vh,
            "vw" => Unit::Vw,
            "tb" => Unit::Tb,
            "%" => Unit::Percent,
            "" => Unit::Unitless,
            _ => Unit::Invalid,
        })
    }
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
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r, self.g, self.b, self.a
        )
    }
}
