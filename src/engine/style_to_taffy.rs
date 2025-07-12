use thiserror::Error;

use super::{
    dom::{AskStyle, SharedNode},
    stylesheet::Value,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("inline is not yet allowed")]
    InlineNotYetAllowed, // this will be added in future (?)
}

// NOTE: commented properties are either irrelevant or will be added in the future
fn get_taffy_from_node(node: SharedNode) -> Result<taffy::Style, Error> {
    taffy::Style {
        display: match node.get_style("display") {
            Value::Keyword(str) => match str.as_str() {
                "inline" => return Err(Error::InlineNotYetAllowed),
                "block" => taffy::Display::Block,
                "flex" => taffy::Display::Flex,
                "grid" => taffy::Display::Grid,
                _ => taffy::Display::Block,
            },
            _ => taffy::Display::Block,
        },
        // item_is_table: todo!(),
        // item_is_replaced: todo!(),
        // box_sizing: todo!(),
        // overflow: todo!(),
        // scrollbar_width: todo!(),
        position: match node.get_style("position") {
            Value::Keyword(str) => match str.as_str() {
                "relative" => taffy::Position::Relative,
                "absolute" => taffy::Position::Absolute,
                _ => taffy::Position::Relative,
            },
            _ => taffy::Position::Relative,
        },
        // TODO: make a helper function / method for this
        inset: taffy::Rect {
            left: match node.get_style_with_fallback("inset", "left") {
                _ => taffy::LengthPercentageAuto::auto(),
            },
            right: todo!(),
            top: todo!(),
            bottom: todo!(),
        },
        size: todo!(),
        min_size: todo!(),
        max_size: todo!(),
        aspect_ratio: todo!(),
        margin: todo!(),
        padding: todo!(),
        border: todo!(),
        align_items: todo!(),
        align_self: todo!(),
        justify_items: todo!(),
        justify_self: todo!(),
        align_content: todo!(),
        justify_content: todo!(),
        gap: todo!(),
        text_align: todo!(),
        flex_direction: todo!(),
        flex_wrap: todo!(),
        flex_basis: todo!(),
        flex_grow: todo!(),
        flex_shrink: todo!(),
        // grid_template_rows: todo!(),
        // grid_template_columns: todo!(),
        // grid_auto_rows: todo!(),
        // grid_auto_columns: todo!(),
        // grid_auto_flow: todo!(),
        // grid_row: todo!(),
        // grid_column: todo!(),
        ..Default::default()
    };
}
