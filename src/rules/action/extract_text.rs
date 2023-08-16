use lopdf::content::Operation;
use crate::collect_text;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct ExtractText {}

impl Operator for ExtractText {
    fn process(&self, context: &mut Context, operation: &Operation) {
        collect_text(&mut context.text, &context.current_encoding, &operation.operands);
    }
}