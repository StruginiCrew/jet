mod error;

use crate::context::{Context, ContextSchema};
use crate::expression::value::Value;
use crate::expression::Expression;
use error::*;

pub struct Runner<'a> {
    context_schema: &'a ContextSchema,
    context: &'a Context,
}

impl<'a> Runner<'a> {
    pub fn new(
        context_schema: &'a ContextSchema,
        context: &'a Context,
    ) -> Result<Self, RunnerError> {
        context_schema.validate(context)?;

        Ok(Runner {
            context_schema,
            context,
        })
    }

    pub fn eval(&self, expression: &Box<dyn Expression>) -> RunnerResult<Value> {
        let value = expression.eval(&self.context)?;

        Ok(value)
    }
}
