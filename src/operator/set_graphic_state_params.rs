use lopdf::content::Operation;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) const OP: &str = "gs";
pub(crate) struct SetGraphicStateParams {}

impl Operator for SetGraphicStateParams {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push('\n');
    }
}