#[derive(Debug)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

pub trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
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

// タプルにも実装ｄけいる
impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

// 記法1
// fn print_point<P: Coordinates>(point: P) { // インライン記法
// 記法2
// fn print_point<P>(point: P) // where記法
// where
//     P: Coordinates
// {
// 記法3
pub fn print_point(point: impl Coordinates) { // impl Trait記法
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

// 複数のトレイト境界を付与
pub fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
    point.clone().to_cartesian()
}

// Pを2回書くにはimpl Trait記法は使用できない
pub fn double_point<P: Coordinates>(point: P) -> P { // インライン記法
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart) // CartesianCoord型からP型に戻す
}

// (T, T)のようにTそのものでない型への制約はwhere記法が必要
pub fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}