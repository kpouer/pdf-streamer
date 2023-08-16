use lopdf::content::Operation;
use crate::rules::operator::Operator;
use crate::text_context::Context;

pub(crate) struct Nothing {}

impl Operator for Nothing {
    fn process(&self, _context: &mut Context, _operation: &Operation) {
    }
}