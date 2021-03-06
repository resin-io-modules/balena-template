use serde_json::json;

use crate::test_eval_eq;

#[test]
fn equal() {
    test_eval_eq!("true == true", json!(true));
    test_eval_eq!("true == false", json!(false));

    test_eval_eq!("1 == 1", json!(true));
    test_eval_eq!("1 == 2", json!(false));

    test_eval_eq!("1.0 == 1", json!(true));
    test_eval_eq!("1.0 == 2", json!(false));

    test_eval_eq!("`abc` == 'abc'", json!(true));
    test_eval_eq!("`abc` == 'ab'", json!(false));
}

#[test]
fn equal_with_filter() {
    test_eval_eq!("`abc` | UPPER == 'ABC'", json!(true));
    test_eval_eq!("`ABC` | LOWER == 'abc'", json!(true));
    test_eval_eq!("`ABC` == 'abc' | UPPER", json!(true));
    test_eval_eq!("`abc` == 'ABC' | LOWER", json!(true));
}

#[test]
fn not_equal() {
    test_eval_eq!("true != true", json!(false));
    test_eval_eq!("true != false", json!(true));

    test_eval_eq!("1 != 1", json!(false));
    test_eval_eq!("1 != 2", json!(true));

    test_eval_eq!("1.0 != 1", json!(false));
    test_eval_eq!("1.0 != 2", json!(true));

    test_eval_eq!("`abc` != 'abc'", json!(false));
    test_eval_eq!("`abc` != 'ab'", json!(true));
}

#[test]
fn not() {
    test_eval_eq!("not false", json!(true));
    test_eval_eq!("not 1 + 1 == 3", json!(true));
}

#[test]
fn greater_than() {
    test_eval_eq!("1 > 1", json!(false));
    test_eval_eq!("1 > 2", json!(false));
    test_eval_eq!("3 > 2", json!(true));

    test_eval_eq!("3.0 > 2", json!(true));
    test_eval_eq!("3 > 2.0", json!(true));
}

#[test]
fn greater_than_or_equal() {
    test_eval_eq!("1 >= 1", json!(true));
    test_eval_eq!("1 >= 2", json!(false));
    test_eval_eq!("3 >= 2", json!(true));

    test_eval_eq!("3.0 >= 2", json!(true));
    test_eval_eq!("3 >= 2.0", json!(true));
}

#[test]
fn lower() {
    test_eval_eq!("1 < 1", json!(false));
    test_eval_eq!("1 < 2", json!(true));
    test_eval_eq!("3 < 2", json!(false));

    test_eval_eq!("3.0 < 2", json!(false));
    test_eval_eq!("3 < 2.0", json!(false));
}

#[test]
fn lower_or_equal() {
    test_eval_eq!("1 <= 1", json!(true));
    test_eval_eq!("1 <= 2", json!(true));
    test_eval_eq!("3 <= 2", json!(false));

    test_eval_eq!("3.0 <= 2", json!(false));
    test_eval_eq!("3 <= 2.0", json!(false));
}

#[test]
fn and() {
    test_eval_eq!("true and true", json!(true));
    test_eval_eq!("true and false", json!(false));

    test_eval_eq!("1 + 1 == 2 and true", json!(true));
    test_eval_eq!("1 + 1 > 1 and 3 < 2", json!(false));
}

#[test]
fn or() {
    test_eval_eq!("true or true", json!(true));
    test_eval_eq!("true or false", json!(true));

    test_eval_eq!("1 + 1 == 2 or false", json!(true));
    test_eval_eq!("1 + 1 > 1 or 3 < 2", json!(true));
}
