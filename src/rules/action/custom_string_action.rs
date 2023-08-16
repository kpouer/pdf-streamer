use lopdf::content::Operation;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct CustomString {
    pub(crate) text: String,
}

impl Operator for CustomString {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push_str(&self.text);
    }
}