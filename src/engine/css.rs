use pest::Parser as _;
use pest_derive::Parser;

use super::stylesheet::{self, Stylesheet};

#[derive(Parser)]
#[grammar = "grammar/css.pest"]
struct CssParser;

#[allow(clippy::result_large_err)]
pub fn parse_from_str(css: &str) -> Result<stylesheet::Stylesheet, pest::error::Error<Rule>> {
    let mut pairs = CssParser::parse(Rule::stylesheet, css)?;

    let sheet = Stylesheet::new(None);

    dbg!(pairs);
    //
    // for pair in pairs {
    //
    // }

    Ok(sheet)
}
