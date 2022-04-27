use crate::expression::ops::*;
use crate::expression::Expression;
use serde_json::{Map as JsonMap, Number as JsonNumber, Value as JsonValue};

type JsonObject = JsonMap<String, JsonValue>;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput { description: String },
    InvalidNumber { description: String },
    EmptyArray,
    MixedArray,
    NestedArray,
    InvalidOp,
    UnknownOp,
}

pub fn parse(input: &str) -> ParseResult<Box<dyn Expression>> {
    let json = match serde_json::from_str(input) {
        Ok(json) => json,
        Err(err) => {
            return Err(ParseError::InvalidInput {
                description: format!("{}", err),
            })
        }
    };

    parse_json_value(json)
}

fn parse_json_value(json: JsonValue) -> ParseResult<Box<dyn Expression>> {
    match json {
        JsonValue::Bool(content) => Ok(bool(content)),
        JsonValue::Number(content) => parse_json_number(content),
        JsonValue::String(content) => Ok(str(content)),
        JsonValue::Array(content) => parse_json_array(content),
        JsonValue::Object(content) => parse_json_object(content),
        _ => panic!("unsupported"),
    }
}

fn parse_json_number(number: JsonNumber) -> ParseResult<Box<dyn Expression>> {
    if number.is_f64() {
        let value = json_number_as_f64(number)?;
        Ok(float(value))
    } else {
        let value = json_number_as_i64(number)?;
        Ok(int(value))
    }
}

fn json_number_as_f64(number: JsonNumber) -> ParseResult<f64> {
    match number.as_f64() {
        Some(value) => Ok(value),
        None => Err(ParseError::InvalidNumber {
            description: format!("Invalid float: {:?}", number),
        }),
    }
}

fn json_number_as_i64(number: JsonNumber) -> ParseResult<i64> {
    match number.as_i64() {
        Some(value) => Ok(value),
        None => Err(ParseError::InvalidNumber {
            description: format!("Invalid int: {:?}", number),
        }),
    }
}

fn parse_json_array(content: Vec<JsonValue>) -> ParseResult<Box<dyn Expression>> {
    if content.len() == 0 {
        return Err(ParseError::EmptyArray);
    }

    match &content[0] {
        JsonValue::Bool(_) => parse_json_bool_array(&content),
        JsonValue::Number(value) => {
            if value.is_f64() {
                parse_json_float_array(&content)
            } else {
                parse_json_int_array(&content)
            }
        }
        JsonValue::String(value) => parse_json_str_array(&content),
        _ => Err(ParseError::NestedArray),
    }
}

fn parse_json_bool_array(content: &Vec<JsonValue>) -> ParseResult<Box<dyn Expression>> {
    let mut items: Vec<bool> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Bool(value) => items.push(*value),
            _ => return Err(ParseError::MixedArray),
        }
    }

    Ok(bool_array(items))
}

fn parse_json_float_array(content: &Vec<JsonValue>) -> ParseResult<Box<dyn Expression>> {
    let mut items: Vec<f64> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Number(value) if value.is_f64() => items.push(value.as_f64().unwrap()),
            _ => return Err(ParseError::MixedArray),
        }
    }

    Ok(float_array(items))
}

fn parse_json_int_array(content: &Vec<JsonValue>) -> ParseResult<Box<dyn Expression>> {
    let mut items: Vec<i64> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Number(value) if value.is_i64() => items.push(value.as_i64().unwrap()),
            _ => return Err(ParseError::MixedArray),
        }
    }

    Ok(int_array(items))
}

fn parse_json_str_array(content: &Vec<JsonValue>) -> ParseResult<Box<dyn Expression>> {
    let mut items: Vec<String> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::String(value) => items.push(value.clone()),
            _ => return Err(ParseError::MixedArray),
        }
    }

    Ok(str_array(items))
}

fn parse_json_object(object: JsonObject) -> ParseResult<Box<dyn Expression>> {
    if object.keys().count() > 1 {
        return Err(ParseError::InvalidOp);
    }

    match object.iter().map(|(k, v)| (k.as_str(), v)).next().unwrap() {
        ("get", JsonValue::Array(content)) if content.len() == 1 => match &content[0] {
            JsonValue::String(name) => Ok(get(name)),
            _ => Err(ParseError::InvalidOp),
        },
        ("eq", JsonValue::Array(content)) if content.len() == 2 => {
            let left = parse_json_value(content[0].clone())?;
            let right = parse_json_value(content[1].clone())?;
            Ok(eq(left, right))
        }
        _ => Err(ParseError::UnknownOp),
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn assert_parse_eq(json: JsonValue) {
        let json_string = json.to_string();

        assert_eq!(
            parse(&json_string).unwrap().to_json().to_string(),
            json_string
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_utils::*;
    use serde_json::json;

    #[test]
    fn it_parses_json() {
        assert_parse_eq(json!({"eq": [1, {"get": ["userId"]}]}))
    }
}
