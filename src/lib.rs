// クレート全体で利用するユーティリティ関数や構造体を定義するモジュール
mod util;

// 数学関連のモジュール
mod math;

// レイ関連のモジュール
mod ray;

mod camera;

pub mod object;

pub mod render;

pub mod platform;

pub mod app;

mod input;



pub mod prelude
{
    pub use crate::app::*;
    pub use crate::camera::*;
    pub use crate::math::*;
    pub use crate::object::*;
    pub use crate::platform::*;
    pub use crate::render::*;
}

mod internal_prelude
{
    pub(crate) use crate::math::*;
    pub(crate) use crate::ray::*;
    pub(crate) use crate::util::*;
}