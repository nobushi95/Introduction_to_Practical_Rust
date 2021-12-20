#[allow(dead_code)]
#[derive(Default)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
    pub stroke_width: u8,
    pub fill: (u8, u8, u8),
    internal_id: String,
}

// 座標
trait Coordinate {}

// デカルト座標
#[allow(dead_code)]
#[derive(Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}
impl Coordinate for CartesianCoord {}

// 極座標
#[allow(dead_code)]
#[derive(Default)]
struct PolarCoord {
    r: f64, 
    theta: f64,
}
impl Coordinate for PolarCoord {}

fn main() {
    let vertexes1 = vec![
        CartesianCoord {x:  0.0, y:  0.0},
        CartesianCoord {x: 50.0, y:  0.0},
        CartesianCoord {x: 30.0, y: 20.0},
    ];

    let vertexes2 = vec![
        PolarCoord { r:  0.0, theta:  0.0 },
        PolarCoord { r: 50.0, theta:  0.0 },
        PolarCoord { r: 30.0, theta: 20.0 },
    ];

    #[allow(unused_variables)]
    let poly1 = Polygon { vertexes: vertexes1, .. Default::default() };
    #[allow(unused_variables)]
    let poly2 = Polygon { vertexes: vertexes2, .. Default::default() };
}
