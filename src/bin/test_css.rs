use std::fs;

use tb::engine::{css, stylesheet::Origin};

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("samples/helloweb/styles.css")?;
    let parsed = css::parse_from_str(&input, Origin::Agent)?;
    dbg!(parsed);
    Ok(())
}
