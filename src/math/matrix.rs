use crate::internal_prelude::*;
use std::ops::Index;



#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Mat4
{
    elements: [[f64;4]; 4],
}

impl Mat4
{
    pub fn identity() -> Mat4
    {
        Mat4
        {
            elements:
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn transpose(&self) -> Self
    {
        Mat4
        {
            elements:
            [
                [self[0][0], self[1][0], self[2][0], self[3][0]],
                [self[0][1], self[1][1], self[2][1], self[3][1]],
                [self[0][2], self[1][2], self[2][2], self[3][2]],
                [self[0][3], self[1][3], self[2][3], self[3][3]]
            ]
        }
    }

    pub fn generate_translation_matrix(p: Point) -> Mat4
    {
        Mat4
        {
            elements:
            [
                [1.0, 0.0, 0.0, p.x()],
                [0.0, 1.0, 0.0, p.y()],
                [0.0, 0.0, 1.0, p.z()],
                [0.0, 0.0, 0.0, 1.0  ]
            ]
        }
    }

    pub fn generate_inverse_translation_matrix(p: Point) -> Mat4
    {
        Mat4
        {
            elements:
            [
                [1.0, 0.0, 0.0, -p.x()],
                [0.0, 1.0, 0.0, -p.y()],
                [0.0, 0.0, 1.0, -p.z()],
                [0.0, 0.0, 0.0, -1.0  ]
            ]
        }
    }

    pub fn generate_scaling_matrix(scale: Vec3) -> Mat4
    {
        Mat4
        {
            elements:
            [
                [scale.x(), 0.0,       0.0,       0.0],
                [0.0,       scale.y(), 0.0,       0.0],
                [0.0,       0.0,       scale.z(), 0.0],
                [0.0,       0.0,       0.0,       1.0]
            ]
        }
    }


    pub fn generate_inverse_scaling_matrix(scale: Vec3) -> Mat4
    {
        Mat4
        {
            elements:
            [
                [1.0 / scale.x(), 0.0,             0.0,             0.0],
                [0.0,             1.0 / scale.y(), 0.0,             0.0],
                [0.0,             0.0,             1.0 / scale.z(), 0.0],
                [0.0,             0.0,             0.0,             1.0]
            ]
        }
    }
}

impl std::ops::Mul for Mat4
{
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output
    {
        let mut result = Mat4::identity();

        for row in 0..4
        {
            for col in 0..4
            {
                let mut sum = 0.0;
                sum += self.elements[row][0] * rhs.elements[0][col];
                sum += self.elements[row][1] * rhs.elements[1][col];
                sum += self.elements[row][2] * rhs.elements[2][col];
                sum += self.elements[row][3] * rhs.elements[3][col];
                result.elements[row][col] = sum;
            }
        }

        result
    }
}

impl std::ops::Mul<(Vec3, f64)> for Mat4
{
    type Output = Vec3;

    fn mul(self, rhs: (Vec3, f64)) -> Self::Output
    {
        let (v, w) : (Vec3, f64) = rhs;
        let (vx, vy, vz) : (f64, f64, f64) = <(f64, f64, f64)>::from(v);
        let x = self.elements[0][0] * vx + self.elements[0][1] * vy + self.elements[0][2] * vz + self.elements[0][3] * w;
        let y = self.elements[1][0] * vx + self.elements[1][1] * vy + self.elements[1][2] * vz + self.elements[1][3] * w;
        let z = self.elements[2][0] * vx + self.elements[2][1] * vy + self.elements[2][2] * vz + self.elements[2][3] * w;

        Vec3::new(x, y, z)
    }
}

impl std::ops::Mul<Point> for Mat4
{
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output
    {
        let product = self * (Vec3::from(rhs), 1.0);
        product.into()
    }
}

impl std::ops::Mul<Direction> for Mat4
{
    type Output = Vec3;

    fn mul(self, rhs: Direction) -> Self::Output
    {
        self * (Vec3::from(rhs), 0.0)
    }
}

impl From<Quaternion> for Mat4
{
    fn from(q: Quaternion) -> Self
    {
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;
        
        Mat4
        {
            elements:
            [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
                [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
                [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }
}

impl Index<usize> for Mat4
{
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.elements[index]
    }
}

impl IntoIterator for Mat4
{
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 16>;

    fn into_iter(self) -> Self::IntoIter
    {
        [
            self.elements[0][0], self.elements[0][1], self.elements[0][2], self.elements[0][3],
            self.elements[1][0], self.elements[1][1], self.elements[1][2], self.elements[1][3],
            self.elements[2][0], self.elements[2][1], self.elements[2][2], self.elements[2][3],
            self.elements[3][0], self.elements[3][1], self.elements[3][2], self.elements[3][3],
        ].into_iter()
    }
}

impl IntoIterator for &Mat4
{
    type Item = f64;
    type IntoIter = std::array::IntoIter<f64, 16>;

    fn into_iter(self) -> Self::IntoIter
    {
        let mat4_copy = *self;
        mat4_copy.into_iter()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn is_same_quaternion(q1: Quaternion, q2: Quaternion) -> bool
    {
        let epsilon = 1.0e-10;
        ((q1.w - q2.w).abs() < epsilon &&
          (q1.x - q2.x).abs() < epsilon &&
          (q1.y - q2.y).abs() < epsilon &&
          (q1.z - q2.z).abs() < epsilon) ||
         ((q1.w + q2.w).abs() < epsilon &&
          (q1.x + q2.x).abs() < epsilon &&
          (q1.y + q2.y).abs() < epsilon &&
          (q1.z + q2.z).abs() < epsilon)
    }

    // 単位行列をPointに掛けてもPointが変わらないことを確認するテスト
    #[test]
    fn test_identity_matrix_keeps_point_unchanged()
    {
        let point: Point = Point::new(1.0, 2.0, 3.0);
        assert_approx_iter_eq_default(Mat4::identity() * point, point);
    }

    // 単位行列をDirectionに掛けてもDirectionが変わらないことを確認するテスト
    #[test]
    fn test_identity_matrix_keeps_direction_unchanged()
    {
        let direction: Direction = Direction::new(0.0, 1.0, 0.0);
        assert_approx_iter_eq_default(Mat4::identity() * direction, direction.into());
    }

    // クオータニオンが回転行列に変換され、更にクオータニオンに変換されたとき、元のクオータニオンと同じ回転を表すことを確認するテスト
    #[test]
    fn test_quaternion_convert_to_matrix_and_back()
    {
        use std::f64::consts::FRAC_1_SQRT_2; // 1/sqrt(2)の近似値
        let test_patterns = [
            (1.0, 0.0, 0.0, 0.0),
            
            (0.0, 1.0, 0.0, 0.0), // X軸周りの180度回転
            (0.0, 0.0, 1.0, 0.0), // Y軸周りの180度回転
            (0.0, 0.0, 0.0, 1.0), // Z軸周りの180度回転

            (0.5, 0.5, 0.5, 0.5), // 120度回転かつ軸が(1,1,1)の回転（エッジケース）
            
            (FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2, 0.0), // Y軸周りの90度回転
            (FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0, 0.0), // X軸周りの90度回転
            (FRAC_1_SQRT_2, 0.0, 0.0, FRAC_1_SQRT_2), // Z軸周りの90度回転
            
            (0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0), // XY軸周りの180度回転
            (0.0, FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2), // XZ軸周りの180度回転
            (0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2), // YZ軸周りの180度回転
        ];

        for mask in 0u8..16
        {
            let sw = if mask & 0b0001 != 0 { 1.0 } else { -1.0 };
            let sx = if mask & 0b0010 != 0 { 1.0 } else { -1.0 };
            let sy = if mask & 0b0100 != 0 { 1.0 } else { -1.0 };
            let sz = if mask & 0b1000 != 0 { 1.0 } else { -1.0 };
            for (w, x, y, z) in test_patterns
            {
                // 比較的単純な回転を表すクオータニオンを選択
                let original_quaternion = Quaternion::from_wxyz(w*sw, x*sx, y*sy, z*sz);
                let converted_quaternion = Quaternion::from(Mat4::from(original_quaternion));
                // クォータニオンは符号が逆でも同じ回転を表すため、両方の可能性をチェック
                let is_approx_equal = is_same_quaternion(original_quaternion, converted_quaternion);
                assert!(is_approx_equal, "Original: {:?}, Converted: {:?}", original_quaternion, converted_quaternion);
            }
        }
    }

    // クオータニオンから回転行列を生成し、意図した回転を実際に生成出来ているか確認するテスト
    #[test]
    fn test_quaternion_rotation_converts_to_expected_matrix_with_simple_cases()
    {
        let test_patterns = [
            (Vec3::unit_y(), 0.0, Point::new(1.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)), // X軸周りの0度回転
            (Vec3::unit_z(), 0.0, Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 0.0)), // Z軸周りの0度回転
            (Vec3::unit_x(), 0.0, Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0)), // Y軸周りの0度回転
            
            (Vec3::unit_y(), FRAC_PI_2, Point::new(0.0, 0.0, 1.0), Point::new(1.0, 0.0, 0.0)), // X軸周りの90度回転
            (Vec3::unit_z(), FRAC_PI_2, Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)), // Z軸周りの90度回転
            (Vec3::unit_x(), FRAC_PI_2, Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 1.0)), // Y軸周りの90度回転
            
            (Vec3::unit_y(), PI, Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, -1.0)), // Y軸周りの180度回転
            (Vec3::unit_z(), PI, Point::new(1.0, 0.0, 0.0), Point::new(-1.0, 0.0, 0.0)), // Z軸周りの180度回転
            (Vec3::unit_x(), PI, Point::new(0.0, 1.0, 0.0), Point::new(0.0, -1.0, 0.0)), // X軸周りの180度回転
            
            (Vec3::unit_y(), PI_2, Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0)), // Y軸周りの360度回転
            (Vec3::unit_z(), PI_2, Point::new(1.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)), // Z軸周りの360度回転
            (Vec3::unit_x(), PI_2, Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 0.0)), // X軸周りの360度回転
        ];

        for (axis, angle, original_point, expected_point) in test_patterns
        {
            for s in (-1..=1).step_by(2)
            {
                let s = s as f64;
                let quaternion = Quaternion::try_from_axis_angle(axis * s, angle * s).unwrap();
                let rotated_point = Mat4::from(quaternion) * original_point;
                assert_approx_iter_eq_default(rotated_point, expected_point);
            }
        }
    }

    #[test]
    fn test_quaternion_rotation_converts_to_expected_matrix_with_complex_cases()
    {
        let test_patterns = [
            (Vec3::unit_y(), FRAC_PI_2, Point::new(0.0, 0.0, 1.0), Point::new(1.0, 0.0, 0.0)), // X軸周りの90度回転
            (Vec3::unit_x(), FRAC_PI_2, Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 1.0)), // Y軸周りの90度回転
            (Vec3::unit_z(), FRAC_PI_2, Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)), // Z軸周りの90度回転
            
            (Vec3::unit_y(), PI, Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, -1.0)), // Y軸周りの180度回転
            (Vec3::unit_x(), PI, Point::new(0.0, 1.0, 0.0), Point::new(0.0, -1.0, 0.0)), // X軸周りの180度回転
            (Vec3::unit_z(), PI, Point::new(1.0, 0.0, 0.0), Point::new(-1.0, 0.0, 0.0)), // Z軸周りの180度回転
            
            (Vec3::unit_y(), PI_2, Point::new(0.0, 0.0, 1.0), Point::new(0.0, 0.0, 1.0)), // Y軸周りの360度回転
            (Vec3::unit_x(), PI_2, Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 0.0)), // X軸周りの360度回転
            (Vec3::unit_z(), PI_2, Point::new(1.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)), // Z軸周りの360度回転
        ];

        for (axis, angle, original_point, expected_point) in test_patterns
        {
            let quaternion = Quaternion::try_from_axis_angle(axis, angle).unwrap();
            let rotated_point = Mat4::from(quaternion) * original_point;
            assert_approx_iter_eq_default(rotated_point, expected_point);
        }
    }

    #[test]
    fn test_translation_affects_point_but_not_direction()
    {
        let translation = Mat4
        {
            elements:
            [
                [1.0, 0.0, 0.0, 3.0],
                [0.0, 1.0, 0.0, -2.0],
                [0.0, 0.0, 1.0, 5.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let point: Point = Point::new(1.0, 2.0, 3.0);
        let direction: Direction = Direction::new(1.0, 0.0, 0.0);

        assert_eq!(translation.clone() * point, Point::new(4.0, 0.0, 8.0));
        assert_eq!(translation * direction, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_matrix_multiplication_composes_transforms_in_order()
    {
        let scale = Mat4
        {
            elements:
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let translation = Mat4
        {
            elements:
            [
                [1.0, 0.0, 0.0, 5.0],
                [0.0, 1.0, 0.0, -1.0],
                [0.0, 0.0, 1.0, 2.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let point = Point::new(1.0, 2.0, 3.0);
        let composed = translation.clone() * scale.clone();

        let expected = translation * (scale * point);
        let actual = composed * point;

        assert_eq!(actual, expected);
    }
}
