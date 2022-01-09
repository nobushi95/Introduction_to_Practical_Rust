use super::Coordinates;
use super::cartesian_coord::CartesianCoord;

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