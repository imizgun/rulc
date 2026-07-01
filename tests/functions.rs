use rulc::core::evaluate_service::EvaluateService;
use rulc::core::repl_output::ReplOutput;

fn svc() -> EvaluateService {
    EvaluateService::new()
}

#[test]
fn define_and_call() {
    let mut s = svc();
    s.evaluate("f(x) = x * 2").unwrap();
    let ReplOutput::Value(v) = s.evaluate("f(5)").unwrap() else {
        panic!()
    };
    assert_eq!(v.to_string(), "10");
}

#[test]
fn zero_arg_fn() {
    let mut s = svc();
    s.evaluate("f() = 5").unwrap();
    let ReplOutput::Value(v) = s.evaluate("f()").unwrap() else {
        panic!()
    };
    assert_eq!(v.to_string(), "5");
}

#[test]
fn nested_calls() {
    let mut s = svc();
    s.evaluate("double(x) = x * 2").unwrap();
    s.evaluate("quad(x) = double(double(x))").unwrap();
    let ReplOutput::Value(v) = s.evaluate("quad(3)").unwrap() else {
        panic!()
    };
    assert_eq!(v.to_string(), "12");
}

#[test]
fn multiarg_fn() {
    let mut s = svc();
    s.evaluate("add(x, y) = x + y").unwrap();
    let ReplOutput::Value(v) = s.evaluate("add(3, 4)").unwrap() else {
        panic!()
    };
    assert_eq!(v.to_string(), "7");
}
