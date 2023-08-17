use lopdf::content::Operation;
use crate::operator::Operator;
use crate::text_context::Context;

pub(crate) struct CustomText {
    pub(crate) text: String,
}

impl Operator for CustomText {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push_str(&self.text);
    }
}