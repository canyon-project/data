use wasm_bindgen::prelude::*;
use serde_json::{Value, Map};

#[wasm_bindgen]
pub fn merge_json(json1: &str, json2: &str) -> String {
    let v1: Value = serde_json::from_str(json1).unwrap();
    let v2: Value = serde_json::from_str(json2).unwrap();

    let mut map1 = v1.as_object().unwrap().clone();
    let map2 = v2.as_object().unwrap().clone();

    for (key, value) in map2 {
        map1.insert(key, value);
    }

    serde_json::to_string(&Value::Object(map1)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn assert_json_eq(json1: &str, json2: &str) {
        let v1: Value = serde_json::from_str(json1).unwrap();
        let v2: Value = serde_json::from_str(json2).unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_merge_json() {
        let json1 = r#"{"name": "John", "age": 30}"#;
        let json2 = r#"{"age": 25, "city": "New York"}"#;
        let expected = r#"{"name": "John", "age": 25, "city": "New York"}"#;
        let result = merge_json(json1, json2);
        assert_json_eq(&result, expected);
    }

    #[test]
    fn test_merge_json_with_empty() {
        let json1 = r#"{}"#;
        let json2 = r#"{"key": "value"}"#;
        let expected = r#"{"key": "value"}"#;
        let result = merge_json(json1, json2);
        assert_json_eq(&result, expected);
    }

    #[test]
    fn test_merge_json_overlapping_keys() {
        let json1 = r#"{"key": "value1"}"#;
        let json2 = r#"{"key": "value2"}"#;
        let expected = r#"{"key": "value2"}"#;
        let result = merge_json(json1, json2);
        assert_json_eq(&result, expected);
    }

    #[test]
    fn test_merge_json_non_overlapping_keys() {
        let json1 = r#"{"key1": "value1"}"#;
        let json2 = r#"{"key2": "value2"}"#;
        let expected = r#"{"key1": "value1", "key2": "value2"}"#;
        let result = merge_json(json1, json2);
        assert_json_eq(&result, expected);
    }
}
