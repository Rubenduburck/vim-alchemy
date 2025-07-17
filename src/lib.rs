use error::Error;
use value::Value;

pub mod classify;
pub mod cli;
pub mod client;
pub mod commands;
pub mod encode;
pub mod error;
pub mod logging;
pub mod value;

fn get_param<T: TryFrom<Value>>(args: &[(Value, Value)], name: &str) -> Result<T, Error> {
    args.iter()
        .find(|(key, _)| key.as_str() == Some(name))
        .map(|(_, value)| T::try_from(value.clone()))
        .ok_or(Error::MissingArgs(name.to_string()))?
        .map_err(|_| Error::InvalidArgs(name.to_string()))
}

fn get_array<T: TryFrom<Value>>(args: &[(Value, Value)], name: &str) -> Result<Vec<T>, Error> {
    args.iter()
        .find(|(key, _)| key.as_str() == Some(name))
        .map(|(_, value)| match value {
            Value::Array(array) => array.iter().flat_map(|v| T::try_from(v.clone())).collect(),
            other => T::try_from(other.clone())
                .map(|value| vec![value])
                .unwrap_or_default(),
        })
        .ok_or(Error::MissingArgs(name.to_string()))
        .map_err(|_| Error::InvalidArgs(name.to_string()))
}
