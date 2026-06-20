#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UnitInterval(f64);

#[derive(Debug)]
pub enum ValidationError {
    NotFinite,
    OutOfRange,
}

impl UnitInterval {
    pub fn try_new(x: f64) -> Result<Self, ValidationError> {
        if !x.is_finite() {
            return Err(ValidationError::NotFinite);
        }
        if !(0.0..=1.0).contains(&x) {
            return Err(ValidationError::OutOfRange);
        }
        Ok(Self(x))
    }

    pub fn get(self) -> f64 { self.0 }
}



#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn test_greater_than_one()
    {
        UnitInterval::try_new(1.5).unwrap();
    }

    #[test]
    #[should_panic(expected = "OutOfRange")]
    fn test_less_than_zero()
    {
        UnitInterval::try_new(-0.5).unwrap();
    }

    #[test]
    #[should_panic(expected = "NotFinite")]
    fn test_not_finite()
    {
        UnitInterval::try_new(f64::INFINITY).unwrap();  
    }
}