use lopdf::content::Operation;
use crate::collect_text;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) const OP: &str = "TJ";
pub(crate) struct ShowText {}

impl Operator for ShowText {
    fn process(&self, context: &mut Context, operation: &Operation) {
        collect_text(&mut context.text, &context.current_encoding, &operation.operands);
        context.text.push(' ');
    }
}