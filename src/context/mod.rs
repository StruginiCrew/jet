pub(crate) mod error;

use crate::expression::eval_type::Type;
use crate::expression::value::Value;
use error::*;
use std::collections::HashMap;

pub type ContextSchemaMismatch = (String, Type, Option<Type>);

#[derive(Clone)]
pub struct ContextSchema {
    schema: HashMap<String, Type>,
}

impl ContextSchema {
    pub fn new() -> Self {
        Self {
            schema: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.schema.get(name)
    }

    pub fn declare<S>(mut self, name: S, eval_type: Type) -> Self
    where
        S: Into<String>,
    {
        self.schema.insert(name.into(), eval_type);
        self
    }

    pub fn validate(&self, context: &Context) -> ContextResult<()> {
        let errors = self.validation_errors(context);
        if errors.len() == 0 {
            Ok(())
        } else {
            Err(ContextError::SchemaMismatch { fields: errors })
        }
    }

    pub fn validation_errors(&self, context: &Context) -> Vec<ContextSchemaMismatch> {
        self.schema
            .iter()
            .filter_map(|(name, eval_type)| match context.get(name) {
                Some(value) if value.concrete_type() == *eval_type => None,
                Some(value) => Some((name.clone(), eval_type.clone(), Some(value.concrete_type()))),
                None => Some((name.clone(), eval_type.clone(), None)),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct Context {
    data: HashMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set_bool<S>(mut self, name: S, value: bool) -> Self
    where
        S: Into<String>,
    {
        self.data.insert(name.into(), Value::Bool(value));
        self
    }

    pub fn set_bool_array<S, V>(mut self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<bool>>,
    {
        self.data
            .insert(name.into(), Value::BoolArray(value.into()));
        self
    }

    pub fn set_int<S>(mut self, name: S, value: i64) -> Self
    where
        S: Into<String>,
    {
        self.data.insert(name.into(), Value::Int(value));
        self
    }

    pub fn set_int_array<S, V>(mut self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<i64>>,
    {
        self.data.insert(name.into(), Value::IntArray(value.into()));
        self
    }

    pub fn set_float<S>(mut self, name: S, value: f64) -> Self
    where
        S: Into<String>,
    {
        self.data.insert(name.into(), Value::Float(value));
        self
    }

    pub fn set_float_array<S, V>(mut self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<f64>>,
    {
        self.data
            .insert(name.into(), Value::FloatArray(value.into()));
        self
    }

    pub fn set_str<S, V>(mut self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<String>,
    {
        self.data.insert(name.into(), Value::Str(value.into()));
        self
    }

    pub fn set_str_array<S, V>(mut self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<String>>,
    {
        self.data.insert(name.into(), Value::StrArray(value.into()));
        self
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }
}
