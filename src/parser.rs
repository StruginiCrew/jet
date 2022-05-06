use crate::expression::ops::*;
use crate::expression::Expression;
use serde_json::{Map as JsonMap, Number as JsonNumber, Value as JsonValue};

type JsonObject = JsonMap<String, JsonValue>;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct ParserError {
    errorKind: ParserErrorKind,
}

#[derive(Debug)]
enum ParserErrorKind {
    InvalidInput { description: String },
    InvalidNumber { description: String },
    EmptyArray,
    MixedArray,
    NestedArray,
    InvalidOp,
    UnknownOp,
}

pub fn parse(input: &str) -> ParserResult<Box<dyn Expression>> {
    let json = match serde_json::from_str(input) {
        Ok(json) => json,
        Err(err) => {
            return Err(ParserError {
                errorKind: ParserErrorKind::InvalidInput {
                    description: format!("{}", err),
                },
            })
        }
    };

    parse_json_value(json)
}

fn parse_json_value(json: JsonValue) -> ParserResult<Box<dyn Expression>> {
    match json {
        JsonValue::Bool(content) => Ok(bool(content)),
        JsonValue::Number(content) => parse_json_number(content),
        JsonValue::String(content) => Ok(str(content)),
        JsonValue::Array(content) => parse_json_array(content),
        JsonValue::Object(content) => parse_json_object(content),
        _ => panic!("unsupported"),
    }
}

fn parse_json_number(number: JsonNumber) -> ParserResult<Box<dyn Expression>> {
    if number.is_f64() {
        let value = json_number_as_f64(number)?;
        Ok(float(value))
    } else {
        let value = json_number_as_i64(number)?;
        Ok(int(value))
    }
}

fn json_number_as_f64(number: JsonNumber) -> ParserResult<f64> {
    match number.as_f64() {
        Some(value) => Ok(value),
        None => Err(ParserError {
            errorKind: ParserErrorKind::InvalidNumber {
                description: format!("Invalid float: {:?}", number),
            },
        }),
    }
}

fn json_number_as_i64(number: JsonNumber) -> ParserResult<i64> {
    match number.as_i64() {
        Some(value) => Ok(value),
        None => Err(ParserError {
            errorKind: ParserErrorKind::InvalidNumber {
                description: format!("Invalid int: {:?}", number),
            },
        }),
    }
}

fn parse_json_array(content: Vec<JsonValue>) -> ParserResult<Box<dyn Expression>> {
    if content.len() == 0 {
        return Err(ParserError {
            errorKind: ParserErrorKind::EmptyArray,
        });
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
        JsonValue::String(_) => parse_json_str_array(&content),
        _ => Err(ParserError {
            errorKind: ParserErrorKind::NestedArray,
        }),
    }
}

fn parse_json_bool_array(content: &Vec<JsonValue>) -> ParserResult<Box<dyn Expression>> {
    let mut items: Vec<bool> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Bool(value) => items.push(*value),
            _ => {
                return Err(ParserError {
                    errorKind: ParserErrorKind::MixedArray,
                })
            }
        }
    }

    Ok(bool_array(items))
}

fn parse_json_float_array(content: &Vec<JsonValue>) -> ParserResult<Box<dyn Expression>> {
    let mut items: Vec<f64> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Number(value) if value.is_f64() => items.push(value.as_f64().unwrap()),
            _ => {
                return Err(ParserError {
                    errorKind: ParserErrorKind::MixedArray,
                })
            }
        }
    }

    Ok(float_array(items))
}

fn parse_json_int_array(content: &Vec<JsonValue>) -> ParserResult<Box<dyn Expression>> {
    let mut items: Vec<i64> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::Number(value) if value.is_i64() => items.push(value.as_i64().unwrap()),
            _ => {
                return Err(ParserError {
                    errorKind: ParserErrorKind::MixedArray,
                })
            }
        }
    }

    Ok(int_array(items))
}

fn parse_json_str_array(content: &Vec<JsonValue>) -> ParserResult<Box<dyn Expression>> {
    let mut items: Vec<String> = Vec::new();

    for json_value in content {
        match json_value {
            JsonValue::String(value) => items.push(value.clone()),
            _ => {
                return Err(ParserError {
                    errorKind: ParserErrorKind::MixedArray,
                })
            }
        }
    }

    Ok(str_array(items))
}

fn parse_json_object(object: JsonObject) -> ParserResult<Box<dyn Expression>> {
    if object.keys().count() > 1 {
        return Err(ParserError {
            errorKind: ParserErrorKind::InvalidOp,
        });
    }

    match object.iter().map(|(k, v)| (k.as_str(), v)).next().unwrap() {
        ("get", JsonValue::Array(content)) if content.len() == 1 => match &content[0] {
            JsonValue::String(name) => Ok(get(name)),
            _ => Err(ParserError {
                errorKind: ParserErrorKind::InvalidOp,
            }),
        },
        ("eq", JsonValue::Array(content)) if content.len() == 2 => {
            let left = parse_json_value(content[0].clone())?;
            let right = parse_json_value(content[1].clone())?;
            Ok(eq(left, right))
        }
        _ => Err(ParserError {
            errorKind: ParserErrorKind::UnknownOp,
        }),
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
