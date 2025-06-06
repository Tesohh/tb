use std::{error::Error, fs};

use tb::engine::{css, stylesheet::Origin};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("samples/helloweb/styles.css")?;
    let parsed = css::parse_from_str(&input, Origin::Agent)?;
    dbg!(parsed);
    Ok(())
}
