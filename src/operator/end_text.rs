use lopdf::content::Operation;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) const OP: &str = "ET";
pub(crate) struct EndText {}

impl Operator for EndText {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        if !context.text.ends_with(' ') {
            context.text.push(' ')
        }
    }
}