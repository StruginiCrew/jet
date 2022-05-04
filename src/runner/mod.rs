pub mod error;

use crate::context::{Context, ContextSchema};
use crate::expression::value::Value;
use crate::expression::Expression;
use error::*;

pub struct Runner {
    context_schema: ContextSchema,
    context: Context,
}

impl Runner {
    pub fn new(context_schema: ContextSchema, context: Context) -> RunnerResult<Self> {
        context_schema.validate(&context)?;

        Ok(Runner {
            context_schema,
            context,
        })
    }

    pub fn update_context(&self, context: Context) -> RunnerResult<()> {
        self.context_schema.validate(&context)?;
        Ok(())
    }

    pub fn eval(&self, expression: &Box<dyn Expression>) -> RunnerResult<Value> {
        let value = expression.eval(&self.context)?;

        Ok(value)
    }
}
