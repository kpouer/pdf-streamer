use lopdf::content::Operation;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) const OP: &str = "TD";
pub(crate) struct MoveText {}

impl Operator for MoveText {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push(' ');
    }
}