use std::fs;

use tb::engine::css;

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("example/helloweb/styles.css")?;
    let parsed = css::parse_from_str(&input)?;
    dbg!(parsed);
    Ok(())
}
