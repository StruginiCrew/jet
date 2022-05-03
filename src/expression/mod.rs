pub mod error;
pub mod eval_type;
pub mod ops;
pub mod value;

use crate::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::value::Value;
use error::*;
use serde_json::{json, Value as JsonValue};

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
        left_type: Type,
        right: Box<dyn Expression>,
    ) {
        assert_eq!(left.eval_type(&context).unwrap(), left_type);
        assert_eq!(left.eval(&context).unwrap(), right.eval(&context).unwrap());
    }

    pub fn assert_eval_type_err(
        context: &Context,
        expression: Box<dyn Expression>,
        expected: Type,
        actual: Type,
    ) {
        let result = expression.eval_type(&context);

        assert!(result.is_err());
        assert!(match result {
            Err(EvalError::TypeMismatch {
                op_json: _,
                arg_position: _,
                expected: err_expected,
                actual: err_actual,
            }) => err_expected == expected && err_actual == actual,
            _ => false,
        })
    }
}
