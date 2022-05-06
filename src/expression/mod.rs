pub mod eval_type;
pub mod ops;
pub mod value;

use crate::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::value::Value;
use serde_json::{json, Value as JsonValue};

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug)]
pub struct EvalError {
    errorKind: EvalErrorKind,
}

#[derive(Debug)]
pub(crate) enum EvalErrorKind {
    MissingContext {
        name: String,
    },
    ValueTypeMismatch {
        expected: Type,
        actual: Type,
    },
    TypeMismatch {
        op_json: JsonValue,
        arg_position: usize,
        expected: Type,
        actual: Type,
    },
}

pub trait Expression {
    fn eval(&self, context: &Context) -> EvalResult<Value>;
    fn eval_bool(&self, context: &Context) -> EvalResult<bool> {
        let value = self.eval(&context)?;

        match value.as_bool() {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::Bool,
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_bool_array(&self, context: &Context, size: usize) -> EvalResult<Vec<bool>> {
        let value = self.eval(&context)?;

        match value.as_bool_array(size) {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::BoolArray(size),
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_int(&self, context: &Context) -> EvalResult<i64> {
        let value = self.eval(&context)?;

        match value.as_int() {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::Int,
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_int_array(&self, context: &Context, size: usize) -> EvalResult<Vec<i64>> {
        let value = self.eval(&context)?;

        match value.as_int_array(size) {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::IntArray(size),
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_float(&self, context: &Context) -> EvalResult<f64> {
        let value = self.eval(&context)?;

        match value.as_float() {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::Float,
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_float_array(&self, context: &Context, size: usize) -> EvalResult<Vec<f64>> {
        let value = self.eval(&context)?;

        match value.as_float_array(size) {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::FloatArray(size),
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_str(&self, context: &Context) -> EvalResult<String> {
        let value = self.eval(&context)?;

        match value.as_str() {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::Str,
                    actual: value.concrete_type(),
                },
            }),
        }
    }
    fn eval_str_array(&self, context: &Context, size: usize) -> EvalResult<Vec<String>> {
        let value = self.eval(&context)?;

        match value.as_str_array(size) {
            Some(content) => Ok(content),
            None => Err(EvalError {
                errorKind: EvalErrorKind::ValueTypeMismatch {
                    expected: Type::StrArray(size),
                    actual: value.concrete_type(),
                },
            }),
        }
    }
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
            Err(EvalError {
                errorKind:
                    EvalErrorKind::TypeMismatch {
                        op_json: _,
                        arg_position: _,
                        expected: err_expected,
                        actual: err_actual,
                    },
            }) => err_expected == expected && err_actual == actual,
            _ => false,
        })
    }
}
