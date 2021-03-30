use serde::de::DeserializeOwned;
use serde_json::Value;

pub(crate) fn parse_params<T: DeserializeOwned>(
    value: Option<Value>,
) -> Result<T, crate::errors::RpcParseError> {
    if let Some(value) = value {
        serde_json::from_value(value)
            .map_err(|err| crate::errors::RpcParseError(format!("Failed parsing args: {}", err)))
    } else {
        Err(crate::errors::RpcParseError("Require at least one parameter".to_owned()))
    }
}

pub(crate) fn from_base64_or_parse_err(
    encoded: String,
) -> Result<Vec<u8>, crate::errors::RpcParseError> {
    near_primitives_core::serialize::from_base64(&encoded)
        .map_err(|err| crate::errors::RpcParseError(err.to_string()))
}
