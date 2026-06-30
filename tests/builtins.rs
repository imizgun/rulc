use rulc::core::evaluate_service::EvaluateService;
use rulc::core::repl_output::ReplOutput;

fn svc() -> EvaluateService { EvaluateService::new() }

fn eval_num(s: &mut EvaluateService, expr: &str) -> f64 {
    let ReplOutput::Value(v) = s.evaluate(expr).unwrap() else { panic!("expected Value for {expr}") };
    v.to_string().parse().unwrap()
}

#[test]
fn constants() {
    let mut s = svc();
    assert!((eval_num(&mut s, "pi") - std::f64::consts::PI).abs() < 1e-9);
    assert!((eval_num(&mut s, "e") - std::f64::consts::E).abs() < 1e-9);
}

#[test]
fn trig() {
    let mut s = svc();
    assert!((eval_num(&mut s, "sin(0)") - 0.0).abs() < 1e-9);
    assert!((eval_num(&mut s, "cos(0)") - 1.0).abs() < 1e-9);
    assert!((eval_num(&mut s, "sin(pi)")).abs() < 1e-9);
}

#[test]
fn sqrt_and_abs() {
    let mut s = svc();
    assert!((eval_num(&mut s, "sqrt(4)") - 2.0).abs() < 1e-9);
    assert!((eval_num(&mut s, "abs(-7)") - 7.0).abs() < 1e-9);
}

#[test]
fn cannot_redefine() {
    let mut s = svc();
    assert!(s.evaluate("sin = 5").is_err());
    assert!(s.evaluate("sin(x) = x").is_err());
    assert!(s.evaluate("pi = 3").is_err());
}
