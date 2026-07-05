use rulc::core::operations::division::DivisionOperation;
use rulc::core::operations::minus::MinusOperation;
use rulc::core::operations::multiply::MultiplyOperation;
use rulc::core::operations::operation::Operation;
use rulc::core::operations::pow::PowOperation;
use rulc::core::operations::sum::SumOperation;

#[test]
fn sum_test_must_be_ok() {
    let sum = SumOperation {};
    let res1 = sum.calc(&[10.0, 2.0]).unwrap();
    let res2 = sum.calc(&[-10.0, 10.0]).unwrap();
    let res3 = sum.calc(&[-100.5, 101.5]).unwrap();

    assert_eq!(12.0, res1);
    assert_eq!(0.0, res2);
    assert_eq!(1.0, res3);
}

#[test]
fn division_test_must_be_ok() {
    let div = DivisionOperation {};
    let res1 = div.calc(&[10.0, 2.0]).unwrap();
    let res2 = div.calc(&[5.0, 2.0]).unwrap();
    let res3 = div.calc(&[1.0, 1.0]).unwrap();

    assert_eq!(5.0, res1);
    assert_eq!(2.5, res2);
    assert_eq!(1.0, res3);
}

#[test]
fn div_by_zero_must_be_err() {
    let div = DivisionOperation {};
    let res1 = div.calc(&[10.0, 0.0]);

    assert!(res1.is_err());
}

#[test]
fn multiply_test_must_be_ok() {
    let mul = MultiplyOperation {};
    let res1 = mul.calc(&[10.0, 2.0]).unwrap();
    let res2 = mul.calc(&[-3.0, -3.0]).unwrap();
    let res3 = mul.calc(&[2.5, 2.0]).unwrap();

    assert_eq!(20.0, res1);
    assert_eq!(9.0, res2);
    assert_eq!(5.0, res3);
}

#[test]
fn multiply_by_zero_must_be_ok() {
    let mul = MultiplyOperation {};
    let res1 = mul.calc(&[123.456, 0.0]).unwrap();

    assert_eq!(0.0, res1);
}

#[test]
fn minus_test_must_be_ok() {
    let minus = MinusOperation {};
    let res1 = minus.calc(&[10.0, 2.0]).unwrap();
    let res2 = minus.calc(&[2.0, 10.0]).unwrap();
    let res3 = minus.calc(&[-5.0, -5.0]).unwrap();

    assert_eq!(8.0, res1);
    assert_eq!(-8.0, res2);
    assert_eq!(0.0, res3);
}

#[test]
fn minus_negative_result_must_be_ok() {
    let minus = MinusOperation {};
    let res1 = minus.calc(&[0.0, 5.0]).unwrap();

    assert_eq!(-5.0, res1);
}

#[test]
fn pow_test_must_be_ok() {
    let pow = PowOperation {};
    let res1 = pow.calc(&[2.0, 8.0]).unwrap();
    let res2 = pow.calc(&[9.0, 0.5]).unwrap();
    let res3 = pow.calc(&[5.0, 0.0]).unwrap();

    assert_eq!(256.0, res1);
    assert_eq!(3.0, res2);
    assert_eq!(1.0, res3);
}

#[test]
fn pow_negative_exponent_must_be_ok() {
    let pow = PowOperation {};
    let res1 = pow.calc(&[2.0, -1.0]).unwrap();

    assert_eq!(0.5, res1);
}
