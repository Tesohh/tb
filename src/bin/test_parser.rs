use pest::{self, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/html.pest"]
struct HtmlParser;

fn main() {
    let parsed = HtmlParser::parse(
        Rule::html,
        r#"
    <br hidden disabled>
    <br hidden='hidden'>
    "#,
    )
    .unwrap();
    println!("{:#?}", parsed);
}
