use balena_temen::engine::context::Context;
use balena_temen::engine::Engine;
use serde_json::json;

macro_rules! test_eval_eq {
    ($e:expr, $c:ident, $r:expr) => {{
        let engine = Engine::default();
        assert_eq!(engine.eval(&$e.parse().unwrap(), &$c).unwrap(), $r);
    }};
}

macro_rules! test_eval_err {
    ($e:expr, $c:ident) => {{
        let engine = Engine::default();
        assert!(engine.eval(&$e.parse().unwrap(), &$c).is_err());
    }};
}

#[test]
fn test_simple() {
    let ctx = Context::new(json!({
        "string": "hallo",
        "integer": 10,
        "float": 3.2,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
    }));

    test_eval_eq!("string", ctx, json!("hallo"));
    test_eval_eq!("integer", ctx, json!(10));
    test_eval_eq!("float", ctx, json!(3.2));
    test_eval_eq!("boolean", ctx, json!(true));
    test_eval_eq!("array", ctx, json!(["a", "b"]));
    test_eval_eq!("object", ctx, json!({"a": "b"}));
    test_eval_eq!("null", ctx, json!(null));

    test_eval_err!("na", ctx);
}

#[test]
fn test_dotted() {
    let ctx = Context::new(json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    }));

    test_eval_eq!("root.another.string", ctx, json!("hallo"));
    test_eval_eq!("root.another.integer", ctx, json!(10));
    test_eval_eq!("root.another.float", ctx, json!(3.2));
    test_eval_eq!("root.another.boolean", ctx, json!(true));
    test_eval_eq!("root.another.array", ctx, json!(["a", "b"]));
    test_eval_eq!("root.another.object", ctx, json!({"a": "b"}));
    test_eval_eq!("root.another.null", ctx, json!(null));

    test_eval_err!("root.another.na", ctx);
    test_eval_err!("root.na", ctx);
    test_eval_err!("na", ctx);
}

#[test]
fn test_square_bracket_string() {
    let ctx = Context::new(json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    }));

    test_eval_eq!("root[`another`][`string`]", ctx, json!("hallo"));
    test_eval_eq!("root[`another`][`integer`]", ctx, json!(10));
    test_eval_eq!("root[`another`][`float`]", ctx, json!(3.2));
    test_eval_eq!("root[`another`][`boolean`]", ctx, json!(true));
    test_eval_eq!("root[`another`][`array`]", ctx, json!(["a", "b"]));
    test_eval_eq!("root[`another`][`object`]", ctx, json!({"a": "b"}));
    test_eval_eq!("root[`another`][`null`]", ctx, json!(null));

    test_eval_err!("root[`another`][`dummy`]", ctx);
    test_eval_err!("root[`dummy`]", ctx);
}

#[test]
fn test_square_bracket_integer() {
    let ctx = Context::new(json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]}));

    test_eval_eq!("root[0]", ctx, json!("hallo"));
    test_eval_eq!("root[1]", ctx, json!(10));
    test_eval_eq!("root[2]", ctx, json!(3.2));
    test_eval_eq!("root[3]", ctx, json!(true));
    test_eval_eq!("root[4]", ctx, json!(["a", "b"]));
    test_eval_eq!("root[5]", ctx, json!({"a": "b"}));
    test_eval_eq!("root[6]", ctx, json!(null));

    test_eval_err!("root[7]", ctx);
}

#[test]
fn test_square_bracket_negative_integer() {
    let ctx = Context::new(json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]}));

    test_eval_eq!("root[-7]", ctx, json!("hallo"));
    test_eval_eq!("root[-6]", ctx, json!(10));
    test_eval_eq!("root[-5]", ctx, json!(3.2));
    test_eval_eq!("root[-4]", ctx, json!(true));
    test_eval_eq!("root[-3]", ctx, json!(["a", "b"]));
    test_eval_eq!("root[-2]", ctx, json!({"a": "b"}));
    test_eval_eq!("root[-1]", ctx, json!(null));

    test_eval_err!("root[-8]", ctx);
}

#[test]
fn test_square_bracket_mixed() {
    let ctx = Context::new(json!({
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]}));

    test_eval_eq!("people[0][`name`]", ctx, json!("Robert"));
    test_eval_eq!("people[0].name", ctx, json!("Robert"));
    test_eval_eq!("people[1][`name`]", ctx, json!("Cyryl"));
    test_eval_eq!("people[1].name", ctx, json!("Cyryl"));
}

#[test]
fn test_square_bracket_indirect() {
    let ctx = Context::new(json!({
        "first": 0,
        "second": 1,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]}));

    test_eval_eq!("people[first][`name`]", ctx, json!("Robert"));
    test_eval_eq!("people[first].name", ctx, json!("Robert"));
    test_eval_eq!("people[second][`name`]", ctx, json!("Cyryl"));
    test_eval_eq!("people[second].name", ctx, json!("Cyryl"));

    test_eval_err!("people[boolean].name", ctx);
    test_eval_err!("people[array].name", ctx);
    test_eval_err!("people[object].name", ctx);
    test_eval_err!("people[null].name", ctx);
}

#[test]
fn test_square_bracket_multiple_indirect() {
    let ctx = Context::new(json!({
        "country": "Czech Republic",
        "city": "Hradec Kralove",
        "data": {
            "country": {
                "Czech Republic": {
                    "city": {
                        "Hradec Kralove": {
                            "rust-developers": 1
                        }
                    }
                }
            }
        }}));

    test_eval_eq!("data.country[country].city[city]['rust-developers']", ctx, json!(1));
}

#[test]
fn test_square_bracket_nested_indirect() {
    let ctx = Context::new(json!({
        "name": "czech",
        "country": {
            "czech": "Czech Republic"
        },
        "data": {
            "country": {
                "Czech Republic": {
                    "rust-developers": 2
                }
            }
        }}));

    test_eval_eq!("data.country[country[name]]['rust-developers']", ctx, json!(2));
}
