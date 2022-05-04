mod context;
mod expression;
mod parser;
mod runner;

pub use context::{Context, ContextSchema};
pub use expression::error::{EvalError, EvalResult};
pub use expression::eval_type::Type;
pub use expression::ops;
pub use expression::value::Value;
pub use expression::Expression;
pub use parser::parse;
pub use runner::error::{RunnerError, RunnerResult};
pub use runner::Runner;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_runs() {
        let schema = ContextSchema::new().declare("userId", Type::Int);
        let context = Context::new().set_int("userId", 1);
        let json = json!({"eq": [1, {"get": ["userId"]}]});

        let expression = parse(&json.to_string()).unwrap();

        let runner = Runner::new(schema, context).unwrap();

        assert_eq!(runner.eval(&expression).unwrap(), Value::Bool(true));
    }
}
