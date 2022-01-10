use super::Coordinates;
use super::cartesian_coord::CartesianCoord;

use crate::linear_transform::LinearTransform;

#[derive(Debug)]
pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

// 極座標は変換が必要
impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl LinearTransform for PolarCoord {
    // デフォルト実装をオーバーロード
    // selfとmut selfのみシグニチャが変わっても良さそう？？
    // selfとmut selfはどちらも所有権がこちらに移動するので大丈夫？
    // selfと&selfや&selfと&mut selfの変更はエラーとなる
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}