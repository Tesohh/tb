use std::collections::HashMap;

use pest::{iterators::Pair, Parser as _};
use pest_derive::Parser;

use super::stylesheet::{self, Dimension, Stylesheet};

#[derive(Parser)]
#[grammar = "grammar/css.pest"]
struct CssParser;

#[allow(clippy::result_large_err)]
pub fn parse_from_str(css: &str) -> Result<stylesheet::Stylesheet, pest::error::Error<Rule>> {
    let pairs = CssParser::parse(Rule::stylesheet, css)?;

    let mut sheet = Stylesheet::new(None);

    for pair in pairs {
        let qualified_rule = match pair.as_rule() {
            Rule::qualified_rule => parse_qualified_rule(pair),
            Rule::EOI => break,
            _ => unreachable!(),
        };

        sheet.rules.push(qualified_rule);
    }

    Ok(sheet)
}

pub fn parse_qualified_rule(pair: Pair<Rule>) -> stylesheet::Rule {
    let mut inner = pair.into_inner();
    let selector = inner.next().unwrap();
    let selector = parse_selector(selector);

    let declarations = inner.next().unwrap();
    let mut decl_map = HashMap::new();
    for declaration in declarations.into_inner() {
        let decls = parse_declaration(declaration);
        dbg!(&decls);
        for (key, value) in decls {
            decl_map.insert(key, value);
        }
    }

    stylesheet::Rule {
        selector,
        declarations: decl_map,
    }
}

pub fn parse_selector(pair: Pair<Rule>) -> stylesheet::Selector {
    let mut selector = stylesheet::Selector {
        compounds: vec![],
        combinators: vec![],
    };

    for compound_or_combinator in pair.into_inner() {
        match compound_or_combinator.as_rule() {
            Rule::compound_selector => {
                let mut compound = stylesheet::CompoundSelector {
                    id: None,
                    tag_name: None,
                    classes: vec![],
                    global: false,
                };

                for selector in compound_or_combinator.into_inner() {
                    match selector.as_rule() {
                        Rule::global_selector => compound.global = true,
                        Rule::id_selector => compound.id = Some(String::from(selector.into_inner().as_str())),
                        Rule::tag_selector => compound.tag_name = Some(String::from(selector.into_inner().as_str())),
                        Rule::class_selector => compound.classes.push(String::from(selector.into_inner().as_str())),
                        _ => unreachable!(),
                    }
                }

                selector.compounds.push(compound);
            },
            Rule::combinator => {
                match compound_or_combinator.as_str() {
                    // TODO: add Space
                    ">" => selector.combinators.push(stylesheet::Combinator::Child),
                    "+" => selector.combinators.push(stylesheet::Combinator::AdjacentSibling),
                    "~" => selector.combinators.push(stylesheet::Combinator::GeneralSibling),
                    _ => unreachable!("wrong combinator"), 
                }
            },
            _ => unreachable!("complex_selector contains something that isn't a compound_selector, got {:?} instead", compound_or_combinator.as_rule())
        }
    }

    selector
}

pub fn parse_declaration(pair: Pair<Rule>) -> Vec<(String, stylesheet::Value)> {
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str().to_string();
    let mut decls = vec![];

    // TEMP:
    let value = inner.next().unwrap();
    let mut value_inner = value.into_inner();

    let inner_next = value_inner.next().unwrap();

    // TEMP: move to separate function
    let value = match inner_next.as_rule() {
        Rule::ident => stylesheet::Value::Keyword(inner_next.as_str().to_string()),
        Rule::dimension => {
            let mut dimension_inner = inner_next.into_inner();
            stylesheet::Value::Dimension(Dimension {
                value: dimension_inner.next().unwrap().as_str().parse().unwrap(),
                unit: stylesheet::Unit::from(dimension_inner.next().unwrap().as_str()),
            })
        }
        _ => unreachable!(),
    };

    decls.push((key, value));

    decls
}
