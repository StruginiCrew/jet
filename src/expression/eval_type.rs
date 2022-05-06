use crate::context::Context;
use crate::expression::{EvalError, EvalErrorKind, EvalResult, Expression};

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Bool,
    BoolArray(usize),
    Int,
    IntArray(usize),
    Float,
    FloatArray(usize),
    Str,
    StrArray(usize),
}

pub fn type_check_all_args_have_same_type(
    context: &Context,
    expression: &dyn Expression,
) -> EvalResult<Option<Type>> {
    let args = expression.args();

    if args.len() == 0 {
        return Ok(None);
    }

    let expected_type = args[0].eval_type(&context)?;

    args.into_iter()
        .enumerate()
        .skip(1)
        .find_map(|(position, arg)| {
            let arg_type = arg.eval_type(&context);

            match arg_type {
                Ok(arg_type) if arg_type == expected_type => None,
                Ok(arg_type) => Some(Err(EvalError {
                    errorKind: EvalErrorKind::TypeMismatch {
                        op_json: expression.to_json(),
                        arg_position: position,
                        expected: expected_type.clone(),
                        actual: arg_type,
                    },
                })),
                Err(err) => Some(Err(err)),
            }
        })
        .unwrap_or(Ok(Some(expected_type)))
}
