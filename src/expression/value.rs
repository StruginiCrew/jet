use crate::context::Context;
use crate::expression::eval_type::Type;
use crate::expression::{EvalResult, Expression};
use serde_json::{json, Value as JsonValue};

pub fn bool_val(content: bool) -> Value {
    Value::Bool(content)
}

pub fn bool(content: bool) -> Box<dyn Expression> {
    Box::new(bool_val(content))
}

pub fn bool_array_val<A>(content: A) -> Value
where
    A: Into<Vec<bool>>,
{
    Value::BoolArray(content.into())
}

pub fn bool_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<bool>>,
{
    Box::new(bool_array_val(content))
}

pub fn int_val(content: i64) -> Value {
    Value::Int(content)
}

pub fn int(content: i64) -> Box<dyn Expression> {
    Box::new(int_val(content))
}

pub fn int_array_val<A>(content: A) -> Value
where
    A: Into<Vec<i64>>,
{
    Value::IntArray(content.into())
}

pub fn int_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<i64>>,
{
    Box::new(int_array_val(content))
}

pub fn float_val(content: f64) -> Value {
    Value::Float(content)
}

pub fn float(content: f64) -> Box<dyn Expression> {
    Box::new(float_val(content))
}

pub fn float_array_val<A>(content: A) -> Value
where
    A: Into<Vec<f64>>,
{
    Value::FloatArray(content.into())
}

pub fn float_array<A>(content: A) -> Box<dyn Expression>
where
    A: Into<Vec<f64>>,
{
    Box::new(float_array_val(content))
}

pub fn str_val<S>(content: S) -> Value
where
    S: Into<String>,
{
    Value::Str(content.into())
}

pub fn str<S>(content: S) -> Box<dyn Expression>
where
    S: Into<String>,
{
    Box::new(str_val(content))
}

pub fn str_array_val<A, S>(content: A) -> Value
where
    S: Into<String>,
    A: Into<Vec<S>>,
{
    Value::StrArray(content.into().into_iter().map(|s| s.into()).collect())
}

pub fn str_array<A, S>(content: A) -> Box<dyn Expression>
where
    S: Into<String>,
    A: Into<Vec<S>>,
{
    Box::new(str_array_val(content))
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

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(content) => Some(*content),
            _ => None,
        }
    }

    pub fn as_bool_array(&self, size: usize) -> Option<Vec<bool>> {
        match self {
            Value::BoolArray(content) => match self.concrete_type() {
                Type::BoolArray(value_size) if value_size == size => Some(content.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(content) => Some(*content),
            _ => None,
        }
    }

    pub fn as_int_array(&self, size: usize) -> Option<Vec<i64>> {
        match self {
            Value::IntArray(content) => match self.concrete_type() {
                Type::IntArray(value_size) if value_size == size => Some(content.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(content) => Some(*content),
            _ => None,
        }
    }

    pub fn as_float_array(&self, size: usize) -> Option<Vec<f64>> {
        match self {
            Value::FloatArray(content) => match self.concrete_type() {
                Type::FloatArray(value_size) if value_size == size => Some(content.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<String> {
        match self {
            Value::Str(content) => Some(content.clone()),
            _ => None,
        }
    }

    pub fn as_str_array(&self, size: usize) -> Option<Vec<String>> {
        match self {
            Value::StrArray(content) => match self.concrete_type() {
                Type::StrArray(value_size) if value_size == size => Some(content.clone()),
                _ => None,
            },
            _ => None,
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
