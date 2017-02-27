#![feature(plugin)]
#![plugin(json_macros)]

#[cfg(feature="with-serde")]
extern crate serde_json;
#[cfg(feature="with-rustc-serialize")]
extern crate rustc_serialize;


#[cfg(feature="with-serde")]
mod imports {
    pub use serde_json::value::Value;
    use serde_json::value::ToJson;
    extern crate serde;
    use self::serde::ser::Serialize;
    
    // convenience fn to avoid re-writing tests, close to serde_json's
    // to_value function.
    // (when https://github.com/serde-rs/json/pull/52 lands this can
    // just re-export serde_json::value:;to_value)
    pub fn to_value<T: ?Sized>(value: &T) -> Value where T: Serialize {
        value.to_json().unwrap()
    }
    
    pub fn empty_map() -> ::serde_json::Map<String,Value> {
        ::serde_json::Map::new()
    }
    
    pub fn to_string(json: &Value) -> String {
        ::serde_json::to_string(&json).unwrap()
    }
    
    pub fn to_string_pretty(json: &Value) -> String{
        ::serde_json::to_string_pretty(&json).unwrap()
    }
}


#[cfg(feature="with-rustc-serialize")]
mod imports {
    pub use rustc_serialize::json::ToJson;
    // convenience renaming for rough serde compatibility
    pub use rustc_serialize::json::Json as Value;
    pub use std::collections::BTreeMap;

    // convenience fn to avoid re-writing tests, close to serde_json's
    // to_value function.
    pub fn to_value<T: ?Sized>(value: &T) -> Value where T: ToJson {
        value.to_json()
    }
    
    pub fn empty_map() -> BTreeMap<String, Value> {
        BTreeMap::new()
    }
    
    pub fn to_string(json: &Value) -> String {
        ::rustc_serialize::json::encode(&json).unwrap()
    }
    
    pub fn to_string_pretty(json: &Value) -> String {
        let mut result = String::new();
        {
            use ::rustc_serialize::Encodable;
            let mut encoder = ::rustc_serialize::json::Encoder::new_pretty(&mut result);
            let _ = json.encode(&mut encoder);
        }
        result
    }
}

use imports::*;


#[test]
fn test_string_lit() {
    #[cfg(feature="with-rustc-serialize")]
    assert_eq!(json!("foo").as_string(), Some("foo"));
    #[cfg(feature="with-serde")]
    assert_eq!(json!("foo").as_str(), Some("foo"));
}

#[test]
fn test_num_lit() {
    assert_eq!(json!(1234).as_i64(), Some(1234));
    assert_eq!(json!(-1234).as_i64(), Some(-1234));
    assert_eq!(json!(12345.).as_f64(), Some(12345.));
    assert_eq!(json!(-12345.6).as_f64(), Some(-12345.6));
}

#[test]
fn test_null_lit() {
    assert!(json!(null).is_null());
}

#[cfg(feature="with-rustc-serialize")]
#[test]
fn test_bool_lit() {
    assert_eq!(json!(true).as_boolean(), Some(true));
    assert_eq!(json!(false).as_boolean(), Some(false));
}

#[cfg(feature="with-serde")]
#[test]
fn test_bool_lit() {
    assert_eq!(json!(true).as_bool(), Some(true));
    assert_eq!(json!(false).as_bool(), Some(false));
}

#[test]
fn test_array_lit() {
    assert_eq!(json!([]), Value::Array(vec![]));
    assert_eq!(json!([null]), Value::Array(vec![to_value(&())]));

    let foobar = Value::Array(vec![to_value("foo"), to_value("bar")]);
    assert_eq!(json!(["foo", "bar"]), foobar);

    let foobar = Value::Array(vec![to_value("foo"),
                                   to_value(&vec![to_value("bar")]),
                                   to_value("baz")]);
    assert_eq!(json!(["foo", ["bar"], "baz"]), foobar);
}

#[test]
fn test_object_lit() {
    let empty = empty_map();
    assert_eq!(json!({}), Value::Object(empty));

    let mut foo_bar = empty_map();
    foo_bar.insert("foo".to_string(), json!("bar"));
    assert_eq!(json!({"foo": "bar"}), Value::Object(foo_bar));

    let mut foo_bar_baz_123 = empty_map();
    foo_bar_baz_123.insert("foo".to_string(), json!("bar"));
    foo_bar_baz_123.insert("baz".to_string(), json!(123));
    assert_eq!(json!({
        "foo": "bar",
        "baz": 123
    }), Value::Object(foo_bar_baz_123));

    let mut nested = empty_map();
    let mut bar_baz = empty_map();
    bar_baz.insert("bar".to_string(), json!("baz"));
    nested.insert("foo".to_string(), Value::Object(bar_baz));
    nested.insert("quux".to_string(), Value::Null);
    assert_eq!(json!({
        "foo": { "bar": "baz" },
        "quux": null
    }), Value::Object(nested));
}

#[test]
fn test_expr_insertion() {
    let hello = "hello world!";
    let json = json!({
        "message": (hello.to_string())
    });
    
    #[cfg(feature="with-rustc-serialize")]
    assert_eq!(json.find("message").and_then(|j| j.as_string()),
               Some(hello));
    #[cfg(feature="with-serde")]
    assert_eq!(json.get("message").and_then(|j| j.as_str()),
               Some(hello));
}

#[test]
fn test_print() {
    let json = json!({
		"message": "hello world!",
		"nested": {
			"number": 12
		}
	});
	println!("JSON IS {:?}",json);
    assert_eq!(to_string(&json), "{\"message\":\"hello world!\",\"nested\":{\"number\":12}}");
    assert_eq!(to_string_pretty(&json),"{\n  \"message\": \"hello world!\",\n  \"nested\": {\n    \"number\": 12\n  }\n}");
}

