use crate::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::value::{str, Value};
use crate::expression::{EvalError, EvalErrorKind, EvalResult, Expression};

pub fn get<S>(name: S) -> Box<dyn Expression>
where
    S: Into<String>,
{
    let name = name.into();

    Box::new(Get {
        name: name.clone(),
        name_arg: str(name),
    })
}

pub struct Get {
    // Get stores its argument as a regular `String` because its `eval_type()` depends
    // on fetching from `Context` by that name. This avoids having to `eval()` inside `eval_type()`.
    name: String,
    // Same value is also stored as `Expression` trait object to satisfy `args()` method return type.
    name_arg: Box<dyn Expression>,
}

impl Expression for Get {
    fn eval(&self, context: &Context) -> EvalResult<Value> {
        match context.get(&self.name) {
            Some(value) => Ok(value.clone()),
            None => Err(EvalError {
                errorKind: EvalErrorKind::MissingContext {
                    name: self.name.clone(),
                },
            }),
        }
    }

    fn eval_type(&self, context: &Context) -> EvalResult<Type> {
        match context.get(&self.name) {
            Some(value) => value.eval_type(&context),
            None => Err(EvalError {
                errorKind: EvalErrorKind::MissingContext {
                    name: self.name.clone(),
                },
            }),
        }
    }

    fn context_dependencies(&self) -> Option<Vec<String>> {
        Some(vec![self.name.clone()])
    }

    fn name(&self) -> &str {
        "get"
    }

    fn args(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.name_arg]
    }
}
