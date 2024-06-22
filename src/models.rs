use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameMod {
    pub id: String,
    pub title: String,
    #[serde(deserialize_with = "string_or_vec")]
    pub link: Vec<String>,
    pub image: Option<Vec<String>>,
    #[serde(deserialize_with = "string_or_vec_option")]
    pub categories: Option<Vec<String>>,
    pub downloads: i32,
    pub author: Option<String>
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    match v {
        Value::String(s) => Ok(vec![s]),
        Value::Array(arr) => {
            let mut vec = Vec::new();
            for item in arr {
                if let Value::String(s) = item {
                    vec.push(s);
                } else {
                    return Err(serde::de::Error::custom("expected a string"));
                }
            }
            Ok(vec)
        },
        _ => Err(serde::de::Error::custom("expected a string or array of strings")),
    }
}

fn string_or_vec_option<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<Value> = Option::deserialize(deserializer)?;

    match v {
        Some(Value::String(s)) => Ok(Some(vec![s])),
        Some(Value::Array(arr)) => {
            let mut vec = Vec::new();
            for item in arr {
                if let Value::String(s) = item {
                    vec.push(s);
                } else {
                    return Err(serde::de::Error::custom("expected a string"));
                }
            }
            Ok(Some(vec))
        },
        Some(_) => Err(serde::de::Error::custom("expected a string or array of strings")),
        None => Ok(None),
    }
}