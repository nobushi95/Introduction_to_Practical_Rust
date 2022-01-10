use super::Coordinates;
use crate::linear_transform::LinearTransform;

#[derive(Debug)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64
}

// デカルト座標はそのまま
impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// LinearTransformトレイトが継承するCoordinatesトレイトをCartesianCoordは実装しているので、LinearTransformを実装できる
impl LinearTransform for CartesianCoord {}