mod expression;
mod parser;

pub use expression::context::Context;
pub use expression::eval_type::Type;
pub use expression::value::Value;
pub use parser::parse;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_parses_and_evals() {
        let mut context = Context::new();
        context.insert("userId".to_string(), Value::Int(1));

        let json = json!({"eq": [1, {"get": ["userId"]}]});

        let expression = parse(&json.to_string()).unwrap();

        assert_eq!(expression.eval(&context).unwrap(), Value::Bool(true));
        assert_eq!(expression.eval_type(&context).unwrap(), Type::Bool);
    }
}
