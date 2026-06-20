use crate::internal_prelude::*;


#[derive(Debug, Clone, Copy)]
pub(crate) struct Ray
{
    origin: Point,
    direction: Direction
}

impl Ray
{
    pub(crate) fn new(origin: Point, direction: Direction) -> Self
    {
        Self { origin, direction }
    }

    pub(crate) fn origin(&self) -> Point
    {
        self.origin
    }

    pub(crate) fn direction(&self) -> Direction
    {
        self.direction
    }

    pub(crate) fn at(&self, t: f64) -> Point
    {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod test
{
    use super::*;


    #[test]
    fn test_ray()
    {
        let point: Point = Point::new(1.0, 2.0, 3.0);
        let direction: Direction = Direction::new(0.0, 1.0, 0.0); 
        let ray = Ray::new(point, direction);
        
        assert!(ray.origin() == point);
        assert!(ray.direction() == direction);
        assert!(ray.at(2.0) == Point::new(1.0, 4.0, 3.0));
    }
}