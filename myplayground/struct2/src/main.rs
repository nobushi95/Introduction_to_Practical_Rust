// 自動導出を使用する場合
// #[derive(Default)] // メンバがDefaultトレイトを実装している必要あり
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

// PolygonへのDefaultトレイトの実装
impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1, // stroke_widthのみ変更
            vertexes: Default::default(),
            fill: Default::default(),
        }
    }
}

fn main() {
    let polygon1: Polygon = Default::default(); // Defaultトレイトの自動導出のデフォルト値となる
    println!("vertexes: {:?}", polygon1.vertexes);
    println!("stroke_width: {}", polygon1.stroke_width);
    println!("fill: {:?}", polygon1.fill);

    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)], // vertexesのみ異なる値
        .. Default::default() // 他はデフォルト
    };
}
