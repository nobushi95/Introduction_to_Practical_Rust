use super::Coordinates;

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