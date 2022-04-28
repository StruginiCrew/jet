use crate::context::Context;
use crate::expression::eval_type::{type_check_all_args_have_same_type, Type};
use crate::expression::value::Value;
use crate::expression::{EvalResult, Expression};

pub fn gt(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Box<dyn Expression> {
    Box::new(Gt { left, right })
}

pub struct Gt {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for Gt {
    fn eval(&self, context: &Context) -> EvalResult<Value> {
        let lval = self.left.eval(context)?;
        let rval = self.right.eval(context)?;

        if lval > rval {
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    }

    fn eval_type(&self, context: &Context) -> EvalResult<Type> {
        type_check_all_args_have_same_type(&context, self)?;
        Ok(Type::Bool)
    }

    fn context_dependencies(&self) -> Option<Vec<String>> {
        None
    }

    fn name(&self) -> &str {
        "gt"
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

        assert_eval_eq(
            &context,
            gt(bool(true), bool(true)),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(bool(true), bool(false)),
            Type::Bool,
            bool(true),
        );
        assert_eval_eq(
            &context,
            gt(bool(false), bool(true)),
            Type::Bool,
            bool(false),
        );
    }

    #[test]
    fn it_compares_bool_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            gt(bool_array([true, false]), bool_array([true, false])),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(bool_array([true, false]), bool_array([false, true])),
            Type::Bool,
            bool(true),
        );
    }

    #[test]
    fn it_compares_ints() {
        let context = Context::new();

        assert_eval_eq(&context, gt(int(1), int(1)), Type::Bool, bool(false));
        assert_eval_eq(&context, gt(int(1), int(2)), Type::Bool, bool(false));
        assert_eval_eq(&context, gt(int(2), int(1)), Type::Bool, bool(true));
    }

    #[test]
    fn it_compares_int_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            gt(int_array([1, 2]), int_array([1, 2])),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(int_array([1, 2]), int_array([2, 1])),
            Type::Bool,
            bool(false),
        );
    }

    #[test]
    fn it_compares_floats() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            gt(float(1.1), float(1.1)),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(float(1.1), float(2.2)),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(&context, gt(float(2.2), float(1.1)), Type::Bool, bool(true));
    }

    #[test]
    fn it_compares_float_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            gt(float_array([1.1, 2.2]), float_array([1.1, 2.2])),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(float_array([1.1, 2.2]), float_array([2.2, 1.1])),
            Type::Bool,
            bool(false),
        );
    }

    #[test]
    fn it_compares_strs() {
        let context = Context::new();

        assert_eval_eq(&context, gt(str("a"), str("a")), Type::Bool, bool(false));
        assert_eval_eq(&context, gt(str("a"), str("b")), Type::Bool, bool(false));
        assert_eval_eq(&context, gt(str("b"), str("a")), Type::Bool, bool(true));
    }

    #[test]
    fn it_compares_str_arrays() {
        let context = Context::new();

        assert_eval_eq(
            &context,
            gt(str_array(["a", "b"]), str_array(["a", "b"])),
            Type::Bool,
            bool(false),
        );
        assert_eval_eq(
            &context,
            gt(str_array(["a", "b"]), str_array(["b", "a"])),
            Type::Bool,
            bool(false),
        );
    }

    #[test]
    fn it_does_not_compare_mixed_types() {
        let context = Context::new();

        assert_eval_type_err(&context, gt(int(1), float(1.0)), Type::Int, Type::Float);
        assert_eval_type_err(&context, gt(bool(true), str("a")), Type::Bool, Type::Str);
        assert_eval_type_err(
            &context,
            gt(float_array([1.1, 2.2]), bool(false)),
            Type::FloatArray(2),
            Type::Bool,
        );
    }
}
