use lopdf::content::Operation;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct CustomChar {
    pub(crate) char: char,
}

impl Operator for CustomChar {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        context.text.push(self.char);
    }
}