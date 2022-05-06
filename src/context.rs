use crate::expression::value::Value;
use std::collections::HashMap;

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
