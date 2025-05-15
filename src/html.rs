use std::collections::HashMap;

use crate::dom::{self, ElementData, Node, NodeType, SharedNode, SharedNodeExt};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/html.pest"]
struct HtmlParser;

#[allow(clippy::result_large_err)]
pub fn parse_from_str(html: &str) -> Result<dom::Dom, pest::error::Error<Rule>> {
    let mut pairs = HtmlParser::parse(Rule::html, html)?;

    let first_token_peek = pairs.peek().unwrap();

    let dom = dom::Dom::new(match first_token_peek.as_rule() {
        Rule::doctype => {
            pairs.next();
            first_token_peek.into_inner().as_str()
        }
        _ => "html",
    });

    for pair in pairs {
        if matches!(pair.as_rule(), Rule::EOI) {
            break;
        }
        let node = parse_node(pair);
        dom.root.append_shared_node(node).unwrap(); // TEMP:unwrap
    }

    Ok(dom)
}

fn parse_node(pair: Pair<Rule>) -> SharedNode {
    match pair.as_rule() {
        Rule::element => {
            let mut inner = pair.into_inner();

            let node = Node::new(NodeType::Element(ElementData::new(
                inner.next().unwrap().as_str(),
                Some(HashMap::new()), // TODO: Parse attrs
            )))
            .to_shared();

            for child in inner {
                let _ = match child.as_rule() {
                    Rule::element => node.append_shared_node(parse_node(child)),
                    Rule::text => node.append_text(child.as_str()),
                    _ => unreachable!(),
                };
            }

            node
        }
        Rule::text => Node::new(NodeType::Text(String::from(pair.as_str()))).to_shared(),

        _ => unreachable!(),
    }
}
