use lopdf::content::Operation;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct OptionalSpace {}

impl Operator for OptionalSpace {
    fn process(&self, context: &mut Context, _operation: &Operation) {
        if !context.text.ends_with(' ') {
            context.text.push(' ')
        }
    }
}