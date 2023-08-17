use lopdf::content::Operation;
use lopdf::Error;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct SetFontAndSize {}

impl Operator for SetFontAndSize {
    fn process(&self, context: &mut Context, operation: &Operation) {
        let current_font = operation
            .operands
            .get(0)
            .ok_or_else(|| Error::Syntax("missing font operand".to_string())).unwrap()
            .as_name().unwrap();
        context.current_encoding = context.encodings.get(current_font).cloned();
    }
}