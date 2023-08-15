use lopdf::content::Operation;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) const OP: &str = "T*";
pub(crate) struct NextLine {}

impl Operator for NextLine {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push('\n');
    }
}