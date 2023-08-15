pub(crate) mod set_font_and_size;
pub(crate) mod set_graphic_state_params;
pub(crate) mod next_line;
pub(crate) mod show_text;
pub(crate) mod show_text_adjusted;
pub(crate) mod end_text;
pub(crate) mod move_text;
pub(crate) mod move_text_set_leading;

use lopdf::content::Operation;
use crate::text_context::Context;

pub trait Operator {
    fn process(&self, context: &mut Context, operation: &Operation);
}