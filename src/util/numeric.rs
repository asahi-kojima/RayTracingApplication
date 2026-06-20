pub(crate) const PI : f64 = std::f64::consts::PI;
pub(crate) const PI_2 : f64 = std::f64::consts::PI * 2.0;
pub(crate) const FRAC_PI_2 : f64 = std::f64::consts::FRAC_PI_2;

#[derive(Debug, Clone, Copy)]
pub enum MathError
{
    DivisionByZero,
    InvalidNumber,
    Overflow,
}

pub fn try_divide(numerator: f64, denominator: f64) -> Result<f64, MathError>
{
    if !numerator.is_finite() || !denominator.is_finite()
    {
        return Err(MathError::InvalidNumber);
    }

    if denominator.abs() < f64::MIN_POSITIVE
    {
        return Err(MathError::DivisionByZero);
    }

    let result = numerator / denominator;
    if !result.is_finite()
    {
        return Err(MathError::Overflow);
    }

    Ok(result)
}

pub fn try_reciprocal(value: f64) -> Result<f64, MathError>
{
    try_divide(1.0, value)
}