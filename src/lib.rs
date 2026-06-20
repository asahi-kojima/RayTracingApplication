// クレート全体で利用するユーティリティ関数や構造体を定義するモジュール
mod util;

// 数学関連のモジュール
mod math;


pub mod prelude
{
    pub use crate::math::*;
}

mod internal_prelude
{
    pub use crate::util::*;
    pub use crate::math::*;
}