use crate::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::{EvalResult, Expression};
use serde_json::{json, Value as JsonValue};

pub fn bool(content: bool) -> Box<dyn Expression> {
    Box::new(Value::Bool(content))
}

pub fn bool_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<bool>>,
{
    Box::new(Value::BoolArray(content.into()))
}

pub fn int(content: i64) -> Box<dyn Expression> {
    Box::new(Value::Int(content))
}

pub fn int_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<i64>>,
{
    Box::new(Value::IntArray(content.into()))
}

pub fn float(content: f64) -> Box<dyn Expression> {
    Box::new(Value::Float(content))
}

pub fn float_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<f64>>,
{
    Box::new(Value::FloatArray(content.into()))
}

pub fn str<S>(content: S) -> Box<dyn Expression>
where
    S: Into<String>,
{
    Box::new(Value::Str(content.into()))
}

pub fn str_array<A, S>(content: A) -> Box<dyn Expression>
where
    S: Into<String>,
    A: Into<Vec<S>>,
{
    Box::new(Value::StrArray(
        content.into().into_iter().map(|s| s.into()).collect(),
    ))
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Value {
    Bool(bool),
    BoolArray(Vec<bool>),
    Int(i64),
    IntArray(Vec<i64>),
    Float(f64),
    FloatArray(Vec<f64>),
    Str(String),
    StrArray(Vec<String>),
}

impl Value {
    pub fn concrete_type(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::BoolArray(content) => Type::BoolArray(content.len()),
            Value::Int(_) => Type::Int,
            Value::IntArray(content) => Type::IntArray(content.len()),
            Value::Float(_) => Type::Float,
            Value::FloatArray(content) => Type::FloatArray(content.len()),
            Value::Str(_) => Type::Str,
            Value::StrArray(content) => Type::StrArray(content.len()),
        }
    }
}

impl Expression for Value {
    fn eval(&self, _context: &Context) -> EvalResult<Value> {
        Ok(self.clone())
    }

    fn eval_type(&self, _context: &Context) -> EvalResult<Type> {
        Ok(self.concrete_type())
    }

    fn context_dependencies(&self) -> Option<Vec<String>> {
        None
    }

    fn name(&self) -> &str {
        match self {
            Value::Bool(_) => "bool",
            Value::BoolArray(_) => "boolArray",
            Value::Int(_) => "int",
            Value::IntArray(_) => "intArray",
            Value::Float(_) => "float",
            Value::FloatArray(_) => "floatArray",
            Value::Str(_) => "str",
            Value::StrArray(_) => "strArray",
        }
    }

    fn args(&self) -> Vec<&Box<dyn Expression>> {
        Vec::new()
    }

    fn to_json(&self) -> JsonValue {
        match self {
            Value::Bool(content) => json!(content),
            Value::BoolArray(content) => json!(content),
            Value::Int(content) => json!(content),
            Value::IntArray(content) => json!(content),
            Value::Float(content) => json!(content),
            Value::FloatArray(content) => json!(content),
            Value::Str(content) => json!(content),
            Value::StrArray(content) => json!(content),
        }
    }
}
