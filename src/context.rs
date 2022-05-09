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

    pub fn set_bool<S>(self, name: S, value: bool) -> Self
    where
        S: Into<String>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::Bool(value));
        Context { data }
    }

    pub fn set_bool_array<S, V>(self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<bool>>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::BoolArray(value.into()));
        Context { data }
    }

    pub fn set_int<S>(self, name: S, value: i64) -> Self
    where
        S: Into<String>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::Int(value));
        Context { data }
    }

    pub fn set_int_array<S, V>(self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<i64>>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::IntArray(value.into()));
        Context { data }
    }

    pub fn set_float<S>(self, name: S, value: f64) -> Self
    where
        S: Into<String>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::Float(value));
        Context { data }
    }

    pub fn set_float_array<S, V>(self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<f64>>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::FloatArray(value.into()));
        Context { data }
    }

    pub fn set_str<S, V>(self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<String>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::Str(value.into()));
        Context { data }
    }

    pub fn set_str_array<S, V>(self, name: S, value: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<String>>,
    {
        let mut data = self.data;
        data.insert(name.into(), Value::StrArray(value.into()));
        Context { data }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.data.get(name)
    }
}
