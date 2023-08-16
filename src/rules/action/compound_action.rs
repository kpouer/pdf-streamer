use lopdf::content::Operation;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct CompoundAction {
    pub(crate) actions: Vec<Box<dyn Operator>>,
}

impl Operator for CompoundAction {
    fn process(&self, context: &mut Context, operation: &Operation) {
        for action in &self.actions {
            action.as_ref().process(context, operation);
        }
    }
}