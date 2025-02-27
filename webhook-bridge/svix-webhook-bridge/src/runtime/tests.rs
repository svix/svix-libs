use super::run_script_inner;
use serde_json::json;
use svix_webhook_bridge_types::JsReturn;

// Really just trying to figure out if the deno runtime is working the way I hope.
#[test]
fn test_happy_fn() {
    let src = r#"
    function handler(input) {
        return { "x": 123, ...input };
    }
    "#
    .to_string();
    let res = run_script_inner(&json!({ "y": 456 }), src).unwrap();
    match res {
        JsReturn::Object(v) => {
            assert_eq!(v["x"].as_i64(), Some(123));
            assert_eq!(v["y"].as_i64(), Some(456));
        }
        JsReturn::Invalid => panic!("got unexpected return value"),
    }
}

#[test]
fn test_invalid_output_bool() {
    let src = r#"
    function handler(input) {
        return false;
    }
    "#
    .to_string();
    let res = run_script_inner(&json!({}), src).unwrap();
    match res {
        JsReturn::Invalid => (),
        JsReturn::Object(_) => panic!("got unexpected return value"),
    }
}

#[test]
// FIXME: serde decodes arrays with keys like "0", "1"... in this situation, failing the test.
#[ignore]
fn test_invalid_output_array() {
    let src = r#"
    function handler(input) {
        return [1, 2];
    }
    "#
    .to_string();
    let res = run_script_inner(&json!({}), src).unwrap();
    match res {
        JsReturn::Invalid => (),
        JsReturn::Object(_) => {
            panic!("got unexpected return value");
        }
    }
}
