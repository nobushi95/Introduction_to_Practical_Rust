mod coordinates;
use coordinates::Coordinates;
use coordinates::print_point;
use coordinates::cartesian_coord::CartesianCoord;
use coordinates::polar_coord::PolarCoord;

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T> // where記法だと具体的な型へ制約がかける
{
    i.convert()
}

fn main() {
    let point = (1.0, 1.0);
    let c = point.to_cartesian();
    println!("c: {:?}", c);

    let p = PolarCoord::from_cartesian(c);
    println!("p = {:?}", p);

    print_point((0.0, 1.0));
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    })
}
