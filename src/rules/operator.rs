use lopdf::content::Operation;
use crate::text_context::Context;

pub(crate) const END_TEXT: &str = "ET";
pub(crate) const SET_GRAPHICS_STATE_PARAMS: &str = "gs";
pub(crate) const MOVE_TEXT: &str = "Td";
pub(crate) const MOVE_TEXT_SET_LEADING: &str = "TD";
pub(crate) const SET_FONT_AND_SIZE: &str = "Tf";
pub(crate) const SHOW_TEXT: &str = "Tj";
pub(crate) const SHOW_TEXT_ADJUSTED: &str = "TJ";
pub(crate) const NEXT_LINE: &str = "T*";

pub trait Operator {
    fn process(&self, context: &mut Context, operation: &Operation);
}