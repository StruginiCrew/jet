use crate::context::error::ContextError;
use crate::context::ContextSchemaMismatch;
use crate::expression::error::EvalError;
use crate::expression::eval_type::Type;
use serde_json::Value as JsonValue;

pub type RunnerResult<T> = Result<T, RunnerError>;

#[derive(Debug, Clone)]
enum RunnerErrorKind {
    ContextSchemaMismatch {
        fields: Vec<ContextSchemaMismatch>,
    },
    ExpressionContextDependencyMissing {
        name: String,
    },
    ExpressionTypeMismatch {
        op_json: JsonValue,
        arg_position: usize,
        expected: Type,
        actual: Type,
    },
}

#[derive(Debug, Clone)]
pub struct RunnerError {
    error: RunnerErrorKind,
}

impl From<EvalError> for RunnerError {
    fn from(item: EvalError) -> Self {
        match item {
            EvalError::TypeMismatch {
                op_json,
                arg_position,
                expected,
                actual,
            } => RunnerError {
                error: RunnerErrorKind::ExpressionTypeMismatch {
                    op_json,
                    arg_position,
                    expected,
                    actual,
                },
            },
            EvalError::MissingContext { name } => RunnerError {
                error: RunnerErrorKind::ExpressionContextDependencyMissing { name },
            },
        }
    }
}

impl From<ContextError> for RunnerError {
    fn from(item: ContextError) -> Self {
        match item {
            ContextError::SchemaMismatch { fields } => RunnerError {
                error: RunnerErrorKind::ContextSchemaMismatch { fields },
            },
        }
    }
}
