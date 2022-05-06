mod context;
mod expression;
mod parser;

pub use context::Context;
pub use expression::eval_type::Type;
pub use expression::ops;
pub use expression::value::Value;
pub use expression::{EvalError, EvalResult, Expression};
pub use parser::{parse, ParserError, ParserResult};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_runs() {
        let context = Context::new().set_int("userId", 1);
        let json = json!({"eq": [1, {"get": ["userId"]}]});

        let expression = parse(&json.to_string()).unwrap();

        assert_eq!(expression.eval(&context).unwrap(), Value::Bool(true));
    }
}
