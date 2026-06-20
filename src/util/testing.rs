use crate::internal_prelude::*;

pub(crate) fn assert_approx_eq(a: f64, b: f64, abs_epsilon: f64, rel_epsilon: f64) {
    assert!(abs_epsilon.is_finite() && abs_epsilon >= 0.0, "abs_epsilon must be finite and >= 0");
    assert!(rel_epsilon.is_finite() && rel_epsilon >= 0.0, "rel_epsilon must be finite and >= 0");
    assert!(a.is_finite() && b.is_finite(), "a and b must be finite");

    let diff = (a - b).abs();
    let scale = a.abs().max(b.abs());
    let tolerance = abs_epsilon.max(rel_epsilon * scale);

    assert!(
        diff <= tolerance,
        "assertion failed: |{} - {}| = {} > tolerance {} (abs_epsilon={}, rel_epsilon={})",
        a, b, diff, tolerance, abs_epsilon, rel_epsilon
    );
}

pub(crate) fn assert_approx_eq_default(a: f64, b: f64) {
    assert_approx_eq(a, b, 1.0e-12, 1.0e-9);
}

pub(crate) fn assert_approx_iter_eq<T: IntoIterator<Item = f64>>(a: T, b: T, abs_epsilon: f64, rel_epsilon: f64) {
    let a_iter = a.into_iter();
    let b_iter = b.into_iter();

    for (a_val, b_val) in a_iter.zip(b_iter) {
        assert_approx_eq(a_val, b_val, abs_epsilon, rel_epsilon);
    }
}

pub(crate) fn assert_approx_iter_eq_default<T: IntoIterator<Item = f64>>(a: T, b: T) {
    let a_iter = a.into_iter();
    let b_iter = b.into_iter();

    assert_approx_iter_eq(a_iter, b_iter, 1.0e-12, 1.0e-9);
}