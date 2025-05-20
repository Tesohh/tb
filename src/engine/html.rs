use std::collections::HashMap;

use crate::engine::dom::{self, ElementData, Node, NodeType, SharedNode, SharedNodeExt};
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
            let tag_or_style = inner.next().unwrap();

            let node = Node::new(NodeType::Element(ElementData::new(
                tag_or_style.as_str(),
                Some(HashMap::new()),
            )))
            .to_shared();

            // unwraps are to crash on poison error
            for child in inner {
                match child.as_rule() {
                    Rule::element => {
                        node.append_shared_node(parse_node(child)).unwrap();
                    }
                    Rule::attr_empty => {
                        node.set_attr(child.as_str(), "").unwrap();
                    }
                    Rule::attr_with_value => {
                        let mut child_inner = child.into_inner();
                        node.set_attr(
                            child_inner.next().unwrap().as_str(),
                            child_inner
                                .next()
                                .unwrap()
                                .as_str()
                                .trim_matches(['\'', '"']),
                        )
                        .unwrap();
                    }
                    Rule::text => {
                        node.append_text(child.as_str()).unwrap();
                    }
                    _ => unreachable!(),
                };
            }

            node
        }
        Rule::text => Node::new(NodeType::Text(String::from(pair.as_str()))).to_shared(),

        _ => unreachable!(),
    }
}
