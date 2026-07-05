use rulc::core::evaluate_service::EvaluateService;
use rulc::core::repl_output::{ReplClearOutput, ReplOutput};

fn svc() -> EvaluateService {
    EvaluateService::new()
}

#[test]
fn clear_memory_removes_user_variables_and_functions() {
    let mut s = svc();
    s.evaluate("x = 5").unwrap();
    s.evaluate("f(y) = y ^ 2").unwrap();

    let ReplOutput::Clear(ReplClearOutput::ClearMemory) = s.evaluate("clear memory").unwrap()
    else {
        panic!()
    };

    assert!(s.evaluate("x").is_err());
    assert!(s.evaluate("f(2)").is_err());
}

#[test]
fn clear_named_variable_removes_only_that_variable() {
    let mut s = svc();
    s.evaluate("x = 5").unwrap();
    s.evaluate("y = 10").unwrap();

    let ReplOutput::Clear(ReplClearOutput::ClearVariable(name)) = s.evaluate("clear x").unwrap()
    else {
        panic!()
    };
    assert_eq!(name, "x");

    assert!(s.evaluate("x").is_err());
    assert!(s.evaluate("y").is_ok());
}

#[test]
fn clear_unknown_name_is_an_error() {
    let mut s = svc();
    assert!(s.evaluate("clear qwerty").is_err());
}

#[test]
fn clear_builtin_is_an_error() {
    let mut s = svc();
    assert!(s.evaluate("clear pi").is_err());
    assert!(s.evaluate("pi").is_ok());
}

#[test]
fn bare_clear_does_not_touch_memory() {
    let mut s = svc();
    s.evaluate("x = 5").unwrap();
    s.evaluate("clear").unwrap();
    assert!(s.evaluate("x").is_ok());
}
