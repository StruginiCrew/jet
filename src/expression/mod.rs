pub mod context;
pub mod eval_type;
pub mod ops;
pub mod value;

use context::Context;
use eval_type::Type;
use serde_json::{json, Value as JsonValue};
use value::Value;

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    MissingContext {
        name: String,
    },
    TypeMismatch {
        op_json: JsonValue,
        arg_position: usize,
        expected: Type,
        got: Type,
    },
}

pub trait Expression {
    fn eval(&self, context: &Context) -> EvalResult<Value>;
    fn eval_type(&self, context: &Context) -> EvalResult<Type>;
    fn context_dependencies(&self) -> Option<Vec<String>>;
    fn name(&self) -> &str;
    fn args(&self) -> Vec<&Box<dyn Expression>>;
    fn to_json(&self) -> JsonValue {
        json!({self.name(): self.args().iter().map(|arg| arg.to_json()).collect::<JsonValue>()})
    }
}

impl std::fmt::Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn assert_eval_eq(
        context: &Context,
        left: Box<dyn Expression>,
        right: Box<dyn Expression>,
    ) -> () {
        assert_eq!(left.eval(&context).unwrap(), right.eval(&context).unwrap())
    }
}
