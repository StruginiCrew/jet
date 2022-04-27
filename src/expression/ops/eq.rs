use crate::expression::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::value::Value;
use crate::expression::{EvalError, EvalResult, Expression};

pub fn eq(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Eq { left, right })
}

pub struct Eq {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Eq {
    fn eval(&self, context: &Context) -> EvalResult<Value> {
        let lval = self.left.eval(context)?;
        let rval = self.right.eval(context)?;

        if lval == rval {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    }

    fn eval_type(&self, context: &Context) -> EvalResult<Type> {
        let ltype = self.left.eval_type(context)?;
        let rtype = self.right.eval_type(context)?;

        if ltype == rtype {
            Ok(Type::Bool)
        } else {
            Err(EvalError::TypeMismatch {
                op_json: self.to_json(),
                arg_position: 2,
                expected: ltype,
                got: rtype,
            })
        }
    }

    fn context_dependencies(&self) -> Option<Vec<String>> {
        None
    }

    fn name(&self) -> &str {
        "eq"
    }

    fn args(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.left, &self.right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::ops::*;
    use crate::expression::test_utils::*;

    #[test]
    fn it_compares_bools() {
        let context = Context::new();

        assert_eval_eq(&context, eq(bool(true), bool(true)), bool(true));
        assert_eval_eq(&context, eq(bool(true), bool(false)), bool(false));
    }

    #[test]
    fn it_compares_bool_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            eq(bool_array([true, false]), bool_array([true, false])),
            bool(true),
        );
        assert_eval_eq(
            &context,
            eq(bool_array([true, false]), bool_array([false, true])),
            bool(false),
        );
    }

    #[test]
    fn it_compares_ints() {
        let context = Context::new();

        assert_eval_eq(&context, eq(int(1), int(1)), bool(true));
        assert_eval_eq(&context, eq(int(1), int(2)), bool(false));
    }

    #[test]
    fn it_compares_int_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            eq(int_array([1, 2]), int_array([1, 2])),
            bool(true),
        );
        assert_eval_eq(
            &context,
            eq(int_array([1, 2]), int_array([2, 1])),
            bool(false),
        );
    }

    #[test]
    fn it_compares_floats() {
        let context = Context::new();

        assert_eval_eq(&context, eq(float(1.1), float(1.1)), bool(true));
        assert_eval_eq(&context, eq(float(1.1), float(2.2)), bool(false));
    }

    #[test]
    fn it_compares_float_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            eq(float_array([1.1, 2.2]), float_array([1.1, 2.2])),
            bool(true),
        );
        assert_eval_eq(
            &context,
            eq(float_array([1.1, 2.2]), float_array([2.2, 1.1])),
            bool(false),
        );
    }

    #[test]
    fn it_compares_strs() {
        let context = Context::new();

        assert_eval_eq(&context, eq(str("a"), str("a")), bool(true));
        assert_eval_eq(&context, eq(str("a"), str("b")), bool(false));
    }

    #[test]
    fn it_compares_str_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            eq(str_array(["a", "b"]), str_array(["a", "b"])),
            bool(true),
        );
        assert_eval_eq(
            &context,
            eq(str_array(["a", "b"]), str_array(["b", "a"])),
            bool(false),
        );
    }
}
