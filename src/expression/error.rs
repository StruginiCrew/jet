use crate::expression::eval_type::Type;
use serde_json::Value as JsonValue;

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
        actual: Type,
    },
}
