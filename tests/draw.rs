use rulc::core::evaluate_service::EvaluateService;
use rulc::core::repl_output::ReplOutput;

fn svc() -> EvaluateService { EvaluateService::new() }

fn points(s: &mut EvaluateService, expr: &str) -> Vec<(f64, f64)> {
    let ReplOutput::FuncPoints { points } = s.evaluate(expr).unwrap() else {
        panic!("expected FuncPoints for {expr}")
    };
    points
}

#[test]
fn user_fn_range() {
    let mut s = svc();
    s.evaluate("f(x) = x * 2").unwrap();
    let pts = points(&mut s, "draw f from 10 to 20");
    assert_eq!(pts[0], (10.0, 20.0));
    assert_eq!(*pts.last().unwrap(), (20.0, 40.0));
}

#[test]
fn negative_range() {
    let mut s = svc();
    s.evaluate("f(x) = x * x").unwrap();
    let pts = points(&mut s, "draw f from -5 to 5");
    assert!((pts[0].0 - (-5.0)).abs() < 1e-9);
    assert!((pts[0].1 - 25.0).abs() < 1e-9);
}

#[test]
fn builtin_fn() {
    let mut s = svc();
    let pts = points(&mut s, "draw sin from 0 to 1");
    assert!(!pts.is_empty());
    assert!((pts[0].1 - 0.0_f64.sin()).abs() < 1e-9);
}

#[test]
fn not_a_function_error() {
    let mut s = svc();
    s.evaluate("x = 5").unwrap();
    assert!(s.evaluate("draw x from 0 to 1").is_err());
}

#[test]
fn unknown_identifier_error() {
    let mut s = svc();
    assert!(s.evaluate("draw unknown from 0 to 1").is_err());
}
