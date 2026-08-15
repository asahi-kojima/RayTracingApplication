use crate::internal_prelude::*;
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion
{
    pub(super) w : f64,
    pub(super) x : f64,
    pub(super) y : f64,
    pub(super) z : f64,
}


impl Quaternion
{
    pub fn identity() -> Quaternion
    {
        Quaternion{w: 1.0, x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn from_wxyz(w: f64, x: f64, y: f64, z: f64) -> Quaternion
    {
        // TODO: x,y,zが正規化されていない可能性を排除出来ないため、チェックを入れるべきかもしれない。
        Quaternion{w, x, y, z}
    }

    pub fn from_axis_angle(axis : Direction, angle : f64) -> Result<Quaternion, MathError>
    {
        let half_angle = angle * 0.5;
        let s = half_angle.sin();

        Ok(Quaternion 
        {
            w: half_angle.cos(),
            x: axis.x() * s,
            y: axis.y() * s,
            z: axis.z() * s,
        })
    }

    pub fn try_from_axis_angle(axis : Vec3, angle : f64) -> Result<Quaternion, MathError>
    {
        let normalized_axis = axis.try_normalize()?;

        let half_angle = angle * 0.5;
        let s = half_angle.sin();

        Ok(Quaternion 
        {
            w: half_angle.cos(),
            x: normalized_axis.x() * s,
            y: normalized_axis.y() * s,
            z: normalized_axis.z() * s,
        })
    }

    pub fn rotate_around_x(angle: f64) -> Quaternion
    {
        let half_angle = angle * 0.5;
        Quaternion{w: half_angle.cos(), x: half_angle.sin(), y: 0.0, z: 0.0}
    }

    pub fn rotate_around_y(angle: f64) -> Quaternion
    {
        let half_angle = angle * 0.5;
        Quaternion{w: half_angle.cos(), x: 0.0, y: half_angle.sin(), z: 0.0}
    }

    pub fn rotate_around_z(angle: f64) -> Quaternion
    {
        let half_angle = angle * 0.5;
        Quaternion{w: half_angle.cos(), x: 0.0, y: 0.0, z: half_angle.sin()}
    }

    pub fn conjugate(self)->Quaternion
    {
        Quaternion{w : self.w, x : -self.x, y : -self.y, z : -self.z}
    }

    pub fn norm_squared(self)->f64
    {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self)->f64
    {
        f64::sqrt(self.norm_squared())
    }

    pub fn normalized(self)->Result<Quaternion, MathError>
    {
        let n = self.norm();

        let reciprocal_n = try_divide(1.0, n)?;

        Ok(Quaternion
        {
            w : self.w * reciprocal_n,
            x : self.x * reciprocal_n,
            y : self.y * reciprocal_n,
            z : self.z * reciprocal_n,
        })
    }

    pub fn rotate_vec3(self, v: Vec3) -> Vec3
    {
        let qv = Quaternion{w : 0.0, x : v.x(), y : v.y(), z : v.z()};
        let qr = self * qv * self.conjugate();
        Vec3::new(qr.x, qr.y, qr.z)
    }

    pub fn rotate_direction(self, v: Direction) -> Direction
    {
        let qv = Quaternion{w : 0.0, x : v.x(), y : v.y(), z : v.z()};
        let qr = self * qv * self.conjugate();
        Direction::new(qr.x, qr.y, qr.z)
    }

    pub fn rotate_point(self, v: Vec3) -> Vec3
    {
        let qv = Quaternion{w : 0.0, x : v.x(), y : v.y(), z : v.z()};
        let qr = self * qv * self.conjugate();
        Vec3::new(qr.x, qr.y, qr.z)
    }
}


impl Mul<Quaternion> for Quaternion
{
    type Output = Quaternion;

    fn mul(self, q: Quaternion) -> Self::Output
    {
        Quaternion
        {
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
            z: self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
        }
    }
}


impl From<Mat4> for Quaternion
{
    fn from(m: Mat4) -> Self
    {
        let trace = m[0][0] + m[1][1] + m[2][2];
        if trace > 0.0
        {
            let s = 0.5 / f64::sqrt(trace + 1.0);
            Quaternion
            {
                w: 0.25 / s,
                x: (m[2][1] - m[1][2]) * s,
                y: (m[0][2] - m[2][0]) * s,
                z: (m[1][0] - m[0][1]) * s,
            }
        }
        else if m[0][0] > m[1][1] && m[0][0] > m[2][2]
        {
            let s = 2.0 * f64::sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]);
            Quaternion
            {
                w: (m[2][1] - m[1][2]) / s,
                x: 0.25 * s,
                y: (m[0][1] + m[1][0]) / s,
                z: (m[0][2] + m[2][0]) / s,
            }
        }
        else if m[1][1] > m[2][2]
        {
            let s = 2.0 * f64::sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]);
            Quaternion
            {
                w: (m[0][2] - m[2][0]) / s,
                x: (m[0][1] + m[1][0]) / s,
                y: 0.25 * s,
                z: (m[1][2] + m[2][1]) / s,
            }
        }
        else
        {
            let s = 2.0 * f64::sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]);
            Quaternion
            {
                w: (m[1][0] - m[0][1]) / s,
                x: (m[0][2] + m[2][0]) / s,
                y: (m[1][2] + m[2][1]) / s,
                z: 0.25 * s,
            }
        }
    }
}
