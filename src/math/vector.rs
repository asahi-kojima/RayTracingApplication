use crate::util::{try_reciprocal, MathError};
use std::{ops::{Add, AddAssign, Sub, SubAssign, Div, Mul, Neg, Deref, Index}};

// =======================================================
// Vec3
// =======================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3
{
    x : f64,
    y : f64,
    z : f64,
}

impl Vec3
{
    pub fn new(x : f64, y : f64, z : f64)->Vec3
    {
        Vec3{x, y, z}
    }

    pub fn to_tuple(&self) -> (f64, f64, f64)
    {
        (self.x, self.y, self.z)
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn zero()->Vec3 { Vec3{x : 0.0, y : 0.0, z : 0.0} }
    pub fn one() -> Vec3 { Vec3{x : 1.0, y : 1.0, z : 1.0} }

    pub fn unit_x() -> Vec3 { Vec3{x : 1.0, y : 0.0, z : 0.0} }
    pub fn unit_y() -> Vec3 { Vec3{x : 0.0, y : 1.0, z : 0.0} }
    pub fn unit_z() -> Vec3 { Vec3{x : 0.0, y : 0.0, z : 1.0} }

    pub fn length_squared(self) -> f64 {self.x * self.x + self.y * self.y + self.z * self.z}
    pub fn length(self) -> f64 {f64::sqrt(self.length_squared())}

    pub fn normalize(self) -> Direction
    {
        let (x, y, z) = self.into();
        let norm: f64 = Vec3::length(self);

        let inv_norm: f64 = 1.0 / norm;

        let x = x * inv_norm;
        let y = y * inv_norm;
        let z = z * inv_norm;
        
        Direction(Vec3{x, y, z})
    }

    pub fn try_normalize(self) -> Result<Direction, MathError>
    {
        let (x, y, z) = self.into();
        let norm: f64 = Vec3::length(self);

        let inv_norm: f64 = try_reciprocal(norm)?;

        let x = x * inv_norm;
        let y = y * inv_norm;
        let z = z * inv_norm;
        
        Ok(Direction(Vec3{x, y, z}))
    }

    pub fn random_in_unit_disk() -> Vec3
    {
    //    use rand::Rng;
    //     let mut rng = rand::rng();

    //     let theta = rng.random_range(0.0..2.0*PI) as f64;
    //     let r = f64::sqrt(rng.random_range(0.0..1.0));
    //     let x = r * theta.cos();
    //     let y = r * theta.sin();

    
    //Vec3::new(x, y, 0.0)
    // TODO
        Vec3::zero()
    }
}

impl From<Vec3> for (f64, f64, f64)
{
    fn from(v: Vec3) -> Self
    {
        (v.x, v.y, v.z)
    }
}

impl From<Point> for Vec3
{
    fn from(point: Point) -> Self
    {
        point.0
    }
}

impl From<Direction> for Vec3
{
    fn from(direction: Direction) -> Self
    {
        direction.0
    }
}

impl Index<usize> for Vec3
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl IntoIterator for Vec3
{
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter
    {
        [self.x, self.y, self.z].into_iter()
    }
}

impl IntoIterator for &Vec3
{
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 3>;

    fn into_iter(self) -> Self::IntoIter
    {
        [self.x, self.y, self.z].into_iter()
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(Vec3);

impl Point
{
    pub fn new(x : f64, y : f64, z : f64)->Point
    {
        Point(Vec3::new(x, y, z))
    }

    pub fn lerp(self, other: Point, t: f64) -> Point
    {
        self + t * (other - self)
    }
}

impl Deref for Point
{
    type Target = Vec3;
    fn deref(&self)->&Self::Target
    {
        &self.0
    }
}

impl From<Vec3> for Point
{
    fn from(v: Vec3) -> Self
    {
        Point(v)
    }
}

impl IntoIterator for Point
{
    type Item = f64;
    type IntoIter = <Vec3 as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.0.into_iter()
    }
}

impl IntoIterator for &Point
{
    type Item = f64;
    type IntoIter = <Vec3 as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.0.into_iter()
    }
}


// =======================================================
// Direction
// =======================================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction(Vec3);// 必ず規格化されていることを保証する。そして規格化を保証するためにフィールドの公開はしない。

impl Direction
{
    pub(crate) fn new(x : f64, y : f64, z : f64)->Direction
    {
        let v = Vec3::new(x, y, z);
        let normalized = v.normalize();
        normalized
    }

    pub fn try_new(x : f64, y : f64, z : f64) -> Result<Direction, MathError>
    {
        let v = Vec3{x, y, z};
        let normalized = v.try_normalize()?;
        Ok(normalized)
    }

    pub fn unit_x() -> Direction{Direction(Vec3::unit_x())}
    pub fn unit_y() -> Direction{Direction(Vec3::unit_y())}
    pub fn unit_z() -> Direction{Direction(Vec3::unit_z())}
}

impl TryFrom<Vec3> for Direction
{
    type Error = MathError;
    fn try_from(v: Vec3) -> Result<Self, MathError>
    {
        v.try_normalize()
    }
}

impl Deref for Direction
{
    type Target = Vec3;
    fn deref(&self)->&Self::Target
    {
        &self.0
    }
}

impl IntoIterator for Direction
{
    type Item = f64;
    type IntoIter = <Vec3 as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.0.into_iter()
    }
}

impl IntoIterator for &Direction
{
    type Item = f64;
    type IntoIter = <Vec3 as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.0.into_iter()
    }
}


// =======================================================
// Mathematical Operations
// =======================================================
// ・Pointの移動
// Point - Point = Vec3
// Point + Vec3 = Point
// Point - Vec3 = Point
// Point + Direction = Point
// Point - Direction = Point
//
// ・可換性のために以下も実装
// Vec3 + Point = Point
// Direction + Point = Point
//
// ベクトルの基本演算
// Vec3 + Vec3 = Vec3
// Vec3 - Vec3 = Vec3
// Vec3 * f64 = Vec3
// f64 * Vec3 = Vec3
// Vec3 / f64 = Vec3
// -Vec3 = Vec3
//
// ・Directionの基本演算
// Direction + Direction = Vec3
// Direction - Direction = Vec3
// Direction * f64 = Vec3
// f64 * Direction = Vec3
// Direction / f64 = Vec3
// -Direction = Direction
//
// DirectionとVec3の混合演算
// Vec3 + Direction = Vec3
// Vec3 - Direction = Vec3
// Direction + Vec3 = Vec3
// Direction - Vec3 = Vec3
//
// ・ベクトルの外積
// Vec3.cross(Vec3) = Vec3
// Direction.cross(Direction) = Vec3
// Vec3.cross(Direction) = Vec3
// Direction.cross(Vec3) = Vec3
//
// ・ベクトルの内積
// Vec3.dot(Vec3) = f64
// Direction.dot(Direction) = f64 
// Vec3.dot(Direction) = f64
// Direction.dot(Vec3) = f64
//
// ・Pointの複合代入演算子
// Point += Vec3
// Point -= Vec3
// Point += Direction
// Point -= Direction
//
// ・Vec3の複合代入演算子
// Vec3 += Vec3
// Vec3 -= Vec3
// Vec3 += Direction
// Vec3 -= Direction
// Vec3 *= f64
// Vec3 /= f64
// =======================================================

// ・Pointの移動
// Point - Point = Vec3
// Point + Vec3 = Point
// Point - Vec3 = Point
// Point + Direction = Point
// Point - Direction = Point
impl Sub<Point> for Point
{
    type Output = Vec3;
    fn sub(self, rhs: Point) -> Self::Output
    {
        Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}
impl Add<Vec3> for Point
{
    type Output = Point;
    fn add(self, rhs: Vec3) -> Self::Output
    {
        Point(Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        })
    }
}
impl Sub<Vec3> for Point
{
    type Output = Point;
    fn sub(self, rhs: Vec3) -> Self::Output
    {
        Point(Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        })
    }
}
impl Add<Direction> for Point
{
    type Output = Point;
    fn add(self, rhs: Direction) -> Self::Output
    {
        Point(Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        })
    }
}
impl Sub<Direction> for Point
{
    type Output = Point;
    fn sub(self, rhs: Direction) -> Self::Output
    {
        Point(Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        })
    }
}


// ・可換性のために以下も実装
// Vec3 + Point = Point
// Direction + Point = Point
impl Add<Point> for Vec3
{
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output
    {
        Point(Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        })
    }
}
impl Add<Point> for Direction
{
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output
    {
        Point(Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        })
    }
}


// ベクトルの基本演算
// Vec3 + Vec3 = Vec3
// Vec3 - Vec3 = Vec3
// Vec3 * f64 = Vec3
// f64 * Vec3 = Vec3
// Vec3 / f64 = Vec3
// -Vec3 = Vec3
impl Add<Vec3> for Vec3
{
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output
    {
        Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        }
    }
}
impl Sub<Vec3> for Vec3
{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output
    {
        Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}
impl Mul<f64> for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output
    {
        Vec3{
            x : self.x * rhs,
            y : self.y * rhs,
            z : self.z * rhs
        }
    }
}
impl Mul<Vec3> for f64
{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output
    {
        Vec3{
            x : self * rhs.x,
            y : self * rhs.y,
            z : self * rhs.z
        }
    }
}
impl Div<f64> for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output
    {
        Vec3{
            x : self.x / rhs,
            y : self.y / rhs,
            z : self.z / rhs
        }
    }
}
impl Neg for Vec3
{
    type Output = Vec3;
    fn neg(self) -> Self::Output
    {
        Vec3{
            x : -self.x,
            y : -self.y,
            z : -self.z
        }
    }
}


// ・Directionの基本演算
// Direction + Direction = Vec3
// Direction - Direction = Vec3
// Direction * f64 = Vec3
// f64 * Direction = Vec3
// Direction / f64 = Vec3
// -Direction = Direction
impl Add<Direction> for Direction
{
    type Output = Vec3;
    fn add(self, rhs: Direction) -> Self::Output
    {
        Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        }
    }
}
impl Sub<Direction> for Direction
{
    type Output = Vec3;
    fn sub(self, rhs: Direction) -> Self::Output
    {
        Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}
impl Mul<f64> for Direction
{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output
    {
        Vec3{
            x : self.x * rhs,
            y : self.y * rhs,
            z : self.z * rhs,
        }
    }
}
impl Mul<Direction> for f64
{
    type Output = Vec3;
    fn mul(self, rhs: Direction) -> Self::Output
    {
        Vec3{
            x : self * rhs.x,
            y : self * rhs.y,
            z : self * rhs.z,
        }
    }
}
impl Div<f64> for Direction
{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output
    {
        Vec3{
            x : self.x / rhs,
            y : self.y / rhs,
            z : self.z / rhs,
        }
    }
}
impl Neg for Direction
{
    type Output = Direction;
    fn neg(self)->Self::Output
    {
        Direction(Vec3{
            x : -self.x,
            y : -self.y,
            z : -self.z,
        })
    }
}


// DirectionとVec3の混合演算
// Vec3 + Direction = Vec3
// Vec3 - Direction = Vec3
// Direction + Vec3 = Vec3
// Direction - Vec3 = Vec3
impl Add<Direction> for Vec3
{
    type Output = Vec3;
    fn add(self, rhs: Direction) -> Self::Output
    {
        Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        }
    }
}
impl Sub<Direction> for Vec3
{
    type Output = Vec3;
    fn sub(self, rhs: Direction) -> Self::Output
    {
        Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}
impl Add<Vec3> for Direction
{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output
    {
        Vec3{
            x : self.x + rhs.x,
            y : self.y + rhs.y,
            z : self.z + rhs.z,
        }
    }
}
impl Sub<Vec3> for Direction
{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output
    {
        Vec3{
            x : self.x - rhs.x,
            y : self.y - rhs.y,
            z : self.z - rhs.z,
        }
    }
}


// ・ベクトルの外積
// Vec3.cross(Vec3) = Vec3
// Direction.cross(Direction) = Vec3
// Vec3.cross(Direction) = Vec3
// Direction.cross(Vec3) = Vec3
impl Vec3
{
    pub fn cross<T : Into<Vec3>>(self, other: T) -> Vec3
    {
        let o = other.into();
        Vec3{
            x : self.y * o.z - self.z * o.y,
            y : self.z * o.x - self.x * o.z,
            z : self.x * o.y - self.y * o.x,
        }
    }
}
fn cross<T1 : Into<Vec3>, T2 : Into<Vec3>>(v0: T1, v1: T2) -> Vec3
{
    let a = v0.into();
    let b = v1.into();
    Vec3{
        x : a.y * b.z - a.z * b.y,
        y : a.z * b.x - a.x * b.z,
        z : a.x * b.y - a.y * b.x,
    }
}


// ・ベクトルの内積
// Vec3.dot(Vec3) = f64
// Direction.dot(Direction) = f64 
// Vec3.dot(Direction) = f64
// Direction.dot(Vec3) = f64
impl Vec3
{
    pub fn dot<T : Into<Vec3>>(self, other: T) -> f64
    {
        let o = other.into();
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}
fn dot<T1 : Into<Vec3>, T2 : Into<Vec3>>(v0: T1, v1: T2) -> f64
{
    let a = v0.into();
    let b = v1.into();
    a.x * b.x + a.y * b.y + a.z * b.z
}


// ・Pointの複合代入演算子
// Point += Vec3
// Point -= Vec3
// Point += Direction
// Point -= Direction
impl AddAssign<Vec3> for Point
{
    fn add_assign(&mut self, rhs: Vec3)
    {
        self.0.x += rhs.x;
        self.0.y += rhs.y;
        self.0.z += rhs.z;
    }
}
impl SubAssign<Vec3> for Point
{
    fn sub_assign(&mut self, rhs: Vec3)
    {
        self.0.x -= rhs.x;
        self.0.y -= rhs.y;
        self.0.z -= rhs.z;
    }
}
impl AddAssign<Direction> for Point
{
    fn add_assign(&mut self, rhs: Direction)
    {
        self.0.x += rhs.x;
        self.0.y += rhs.y;
        self.0.z += rhs.z;
    }
}
impl SubAssign<Direction> for Point
{
    fn sub_assign(&mut self, rhs: Direction)
    {
        self.0.x -= rhs.x;
        self.0.y -= rhs.y;
        self.0.z -= rhs.z;
    }
}


// ・Vec3の複合代入演算子
// Vec3 += Vec3
// Vec3 -= Vec3
// Vec3 += Direction
// Vec3 -= Direction
// Vec3 *= f64
// Vec3 /= f64
impl AddAssign<Vec3> for Vec3
{
    fn add_assign(&mut self, rhs: Vec3)
    {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign<Vec3> for Vec3
{
    fn sub_assign(&mut self, rhs: Vec3)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl AddAssign<Direction> for Vec3
{
    fn add_assign(&mut self, rhs: Direction)
    {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign<Direction> for Vec3
{
    fn sub_assign(&mut self, rhs: Direction)
    {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl std::ops::MulAssign<f64> for Vec3
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl std::ops::DivAssign<f64> for Vec3
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}


// =======================================================
// Unit Tests
// =======================================================
#[cfg(test)]
mod tests 
{
    use super::*;

    // ・Pointの移動
    // Point - Point = Vec3
    // Point + Vec3 = Point
    // Point - Vec3 = Point
    // Point + Direction = Point
    // Point - Direction = Point
    #[test]
    fn test_point_sub_point()
    {
        let p1 = Point::new(3.0, 4.0, 5.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        let result = p1 - p2;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }

    #[test]
    fn test_point_add_vec3()
    {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::new(2.0, 3.0, 4.0);
        let result = p + v;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    fn test_point_sub_vec3()
    {
        let p = Point::new(5.0, 6.0, 7.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = p - v;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn test_point_add_direction()
    {
        let p = Point::new(1.0, 2.0, 3.0);
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = p + d;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_point_sub_direction()
    {
        let p = Point::new(5.0, 6.0, 7.0);
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = p - d;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 7.0);
    }

    // ・可換性のために以下も実装
    // Vec3 + Point = Point
    // Direction + Point = Point
    #[test]
    fn test_vec3_add_point()
    {
        let v = Vec3::new(2.0, 3.0, 4.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let result = v + p;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 5.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    fn test_direction_add_point()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let p = Point::new(1.0, 2.0, 3.0);
        let result = d + p;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    // ベクトルの基本演算
    // Vec3 + Vec3 = Vec3
    // Vec3 - Vec3 = Vec3
    // Vec3 * f64 = Vec3
    // f64 * Vec3 = Vec3
    // Vec3 / f64 = Vec3
    // -Vec3 = Vec3
    #[test]
    fn test_vec3_add_vec3()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vec3_sub_vec3()
    {
        let v1 = Vec3::new(5.0, 6.0, 7.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn test_vec3_mul_f64()
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_f64_mul_vec3()
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = 3.0 * v;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vec3_div_f64()
    {
        let v = Vec3::new(4.0, 6.0, 8.0);
        let result = v / 2.0;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 3.0);
        assert_eq!(result.z, 4.0);
    }

    #[test]
    fn test_neg_vec3()
    {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let result = -v;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -3.0);
    }

    // ・Directionの基本演算
    // Direction + Direction = Vec3
    // Direction - Direction = Vec3
    // Direction * f64 = Vec3
    // f64 * Direction = Vec3
    // Direction / f64 = Vec3
    // -Direction = Direction
    #[test]
    fn test_direction_add_direction()
    {
        let d1 = Direction::new(1.0, 0.0, 0.0);
        let d2 = Direction::new(0.0, 1.0, 0.0);
        let result = d1 + d2;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_direction_sub_direction()
    {
        let d1 = Direction::new(1.0, 0.0, 0.0);
        let d2 = Direction::new(0.0, 1.0, 0.0);
        let result = d1 - d2;
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_direction_mul_f64()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = d * 2.0;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_f64_mul_direction()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = 3.0 * d;
        assert_eq!(result.x, 3.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_direction_div_f64()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = d / 2.0;
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_neg_direction()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = -d;
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    // DirectionとVec3の混合演算
    // Vec3 + Direction = Vec3
    // Vec3 - Direction = Vec3
    // Direction + Vec3 = Vec3
    // Direction - Vec3 = Vec3
    #[test]
    fn test_vec3_add_direction()
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = v + d;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_vec3_sub_direction()
    {
        let v = Vec3::new(5.0, 6.0, 7.0);
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = v - d;
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
        assert_eq!(result.z, 7.0);
    }

    #[test]
    fn test_direction_add_vec3()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = d + v;
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_direction_sub_vec3()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = d - v;
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    // ・ベクトルの外積
    // Vec3.cross(Vec3) = Vec3
    // Direction.cross(Direction) = Vec3
    // Vec3.cross(Direction) = Vec3
    // Direction.cross(Vec3) = Vec3
    #[test]
    fn test_vec3_cross_vec3()
    {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let result = v1.cross(v2);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_direction_cross_direction()
    {
        let d1 = Direction::new(1.0, 0.0, 0.0);
        let d2 = Direction::new(0.0, 1.0, 0.0);
        let result = d1.cross(d2);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_vec3_cross_direction()
    {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let d = Direction::new(0.0, 1.0, 0.0);
        let result = v.cross(d);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_direction_cross_vec3()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let result = d.cross(v);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_cross_anticommutative()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result1 = v1.cross(v2);
        let result2 = v2.cross(v1);
        assert_eq!(result1.x, -result2.x);
        assert_eq!(result1.y, -result2.y);
        assert_eq!(result1.z, -result2.z);
    }

    // ・ベクトルの内積
    // Vec3.dot(Vec3) = f64
    // Direction.dot(Direction) = f64 
    // Vec3.dot(Direction) = f64
    // Direction.dot(Vec3) = f64
    #[test]
    fn test_vec3_dot_vec3()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.dot(v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_direction_dot_direction()
    {
        let d1 = Direction::new(1.0, 0.0, 0.0);
        let d2 = Direction::new(0.0, 1.0, 0.0);
        let result = d1.dot(d2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_vec3_dot_direction()
    {
        let v = Vec3::new(2.0, 0.0, 0.0);
        let d = Direction::new(1.0, 0.0, 0.0);
        let result = v.dot(d);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_direction_dot_vec3()
    {
        let d = Direction::new(1.0, 0.0, 0.0);
        let v = Vec3::new(3.0, 0.0, 0.0);
        let result = d.dot(v);
        assert_eq!(result, 3.0);
    }

    // ・Pointの複合代入演算子
    // Point += Vec3
    // Point -= Vec3
    // Point += Direction
    // Point -= Direction
    #[test]
    fn test_point_add_asign_vec3()
    {
        let mut p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::new(2.0, 3.0, 4.0);
        p += v;
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 5.0);
        assert_eq!(p.z, 7.0);
    }

    #[test]
    fn test_point_sub_asign_vec3()
    {
        let mut p = Point::new(1.0, 2.0, 3.0);
        let v = Vec3::new(2.0, 3.0, 4.0);
        p -= v;
        assert_eq!(p.x, -1.0);
        assert_eq!(p.y, -1.0);
        assert_eq!(p.z, -1.0);
    }

    #[test]
    fn test_point_add_asign_direction()
    {
        let mut p = Point::new(1.0, 2.0, 3.0);
        let d= Direction(Vec3{x:1.0, y:0.0, z:0.0});
        p += d;
        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_point_sub_asign_direction()
    {
        let mut p = Point::new(1.0, 2.0, 3.0);
        let d = Direction::new(0.0, 1.0, 0.0);
        p -= d;
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 1.0);
        assert_eq!(p.z, 3.0);
    }

    // ・Vec3の複合代入演算子
    // Vec3 += Vec3
    // Vec3 -= Vec3
    // Vec3 += Direction
    // Vec3 -= Direction
    // Vec3 *= f64
    // Vec3 /= f64
    #[test]
    fn test_vec3_add_assign_vec3()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);
    }

    #[test]
    fn test_vec3_sub_assign_vec3()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        v1 -= v2;
        assert_eq!(v1.x, -3.0);
        assert_eq!(v1.y, -3.0);
        assert_eq!(v1.z, -3.0);
    }

    #[test]
    fn test_vec3_add_assign_direction()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let d = Direction::new(1.0, 1.0, 1.0);
        v1 += d;
        assert_eq!(v1.x, 1.0 + 1.0 / f64::sqrt(3.0));
        assert_eq!(v1.y, 2.0 + 1.0 / f64::sqrt(3.0));
        assert_eq!(v1.z, 3.0 + 1.0 / f64::sqrt(3.0));
    }

    #[test]
    fn test_vec3_sub_assign_direction()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let d = Direction::new(1.0, 1.0, 1.0);
        v1 -= d;
        assert_eq!(v1.x, 1.0 - 1.0 / f64::sqrt(3.0));
        assert_eq!(v1.y, 2.0 - 1.0 / f64::sqrt(3.0));
        assert_eq!(v1.z, 3.0 - 1.0 / f64::sqrt(3.0));
    }

    #[test]
    fn test_vec3_mul_assign_f64()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let f = 2.0;
        v1 *= f;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_vec3_div_assign_f64()
    {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let f = 2.0;
        v1 /= f;
        assert_eq!(v1.x, 0.5);
        assert_eq!(v1.y, 1.0);
        assert_eq!(v1.z, 1.5);
    }
}