use std::{collections::HashMap, str::FromStr};

use pest::{iterators::Pair, Parser as _};
use pest_derive::Parser;

use super::stylesheet::{self, Dimension, Stylesheet};

#[derive(Parser)]
#[grammar = "grammar/css.pest"]
pub struct CssParser;

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
        for (key, value) in decls {
            decl_map.insert(key, value);
        }
    }

    stylesheet::Rule {
        selector,
        props: decl_map,
    }
}

pub fn parse_selector(pair: Pair<Rule>) -> stylesheet::ComplexSelector {
    let mut selector = stylesheet::ComplexSelector {
        inner: vec![],
        combinators: vec![],
    };

    for compound_or_combinator in pair.into_inner() {
        match compound_or_combinator.as_rule() {
            Rule::compound_selector => {
                let mut compound = stylesheet::Selector {
                    id: None,
                    tag_name: None,
                    classes: vec![],
                };

                for selector in compound_or_combinator.into_inner() {
                    match selector.as_rule() {
                        Rule::global_selector => {},
                        Rule::id_selector => compound.id = Some(String::from(selector.into_inner().as_str())),
                        Rule::tag_selector => compound.tag_name = Some(String::from(selector.into_inner().as_str())),
                        Rule::class_selector => compound.classes.push(String::from(selector.into_inner().as_str())),
                        _ => unreachable!(),
                    }
                }

                selector.inner.push(compound);
            },
            Rule::combinator => {
                match compound_or_combinator.as_str().trim() {
                    "" => selector.combinators.push(stylesheet::Combinator::Descendant),
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

    // TEMP: should support multi value syntax
    let value = parse_value(inner.next().unwrap());

    decls.push((key, value));

    decls
}

pub fn parse_value(value: Pair<Rule>) -> stylesheet::Value {
    if !matches!(value.as_rule(), Rule::value) {
        unreachable!("bad developer should not have passed a non Rule::value to parse_value!!");
    }

    let inner = value.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::ident => stylesheet::Value::Keyword(inner.as_str().to_string()),
        Rule::dimension => {
            let mut dimension_inner = inner.into_inner();
            stylesheet::Value::Dimension(Dimension {
                value: dimension_inner.next().unwrap().as_str().parse().unwrap(),
                unit: stylesheet::Unit::from_str(dimension_inner.next().unwrap().as_str()).unwrap(),
            })
        }
        _ => unreachable!(),
    }
}
