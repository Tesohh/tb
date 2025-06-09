use std::error::Error;

use tb::engine::{
    dom::{shared_node, AskStyle},
    stylesheet::{Origin, Value},
};

fn main() -> Result<(), Box<dyn Error>> {
    let html_input = String::from(
        r#"
        <h1>Cissy</h1>
        <p class="yellow">iojwefijo</p>
        <p style="color: pink">iojwefijo</p>
        <div style="color: cyan">
            <span id="spannen"></span>
        </div>
        <div class="wide">
            <div class="half-as-wide">
                <div class="half-as-wide"></div>
            </div>
        </div>
    "#,
    );

    let css_input_agent = String::from(
        r#"
        h1 {
            color: red;
        }

        p.yellow {
            color: yellow;
        }
    "#,
    );

    let css_input_author = String::from(
        r#"
        h1 {
            color: blue;
        }

        .yellow {
            color: purple;
        }

        .wide {
            width: 100vw;
            height: 50vh;
        }

        .half-as-wide {
            width: 50%;
            height: 10%;
        }
    "#,
    );

    let sheet_agent = tb::engine::css::parse_from_str(&css_input_agent, Origin::Agent)?;
    let sheet_author = tb::engine::css::parse_from_str(&css_input_author, Origin::Author)?;

    let mut dom = tb::engine::html::parse_from_str(&html_input)?;

    dom.apply_stylesheet(sheet_agent)?;
    dom.apply_stylesheet(sheet_author)?;

    let node = dom.query_select(".half-as-wide>.half-as-wide")?[0].clone();
    let width = node.ask_style("width")?.unwrap();
    dbg!(&width);

    let Value::Dimension(dim) = width.value.value else {
        unreachable!()
    };

    let parent = node
        .read()
        .or(Err(shared_node::Error::Poison))?
        .parent
        .clone()
        .ok_or(shared_node::Error::Unreachable(
            shared_node::UnreachableError::NoParent,
        ))?
        .upgrade()
        .ok_or(shared_node::Error::MissingParentUpgrade)?;

    dbg!(parent.ask_style("width"));

    dbg!(dim.as_tb(&parent, "width", (120, 60)));

    // dbg!(dom.root);

    Ok(())
}
