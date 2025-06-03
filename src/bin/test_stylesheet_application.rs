use tb::engine::{dom::AskStyle, stylesheet::Origin};

fn main() -> anyhow::Result<()> {
    let html_input = String::from(
        r#"
        <h1>Cissy</h1>
        <p class="yellow">iojwefijo</p>
        <p style="color: pink">iojwefijo</p>
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
    "#,
    );

    let sheet_agent = tb::engine::css::parse_from_str(&css_input_agent, Origin::Agent)?;
    let sheet_author = tb::engine::css::parse_from_str(&css_input_author, Origin::Author)?;

    let mut dom = tb::engine::html::parse_from_str(&html_input)?;

    dom.apply_stylesheet(sheet_agent)?;
    dom.apply_stylesheet(sheet_author)?;

    dbg!(dom.query_select("p")?[0].ask_style("color")?);

    // dbg!(dom.root);

    Ok(())
}
