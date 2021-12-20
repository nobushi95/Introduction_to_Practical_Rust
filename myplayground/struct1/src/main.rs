struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon { vertexes, stroke_width, fill} // 宣言と同じ順番に置く必要あり
}

fn main() {
    // 通常の初期化
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255), // 順不同
        stroke_width: 1,
    };

    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // パターンマッチによりフィールドにアクセス
    let Polygon { vertexes: quad_vx, .. } = quadrangle; // .. で他は省略可能
    assert_eq!(quad_vx, vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // フィールドと同じ名前の変数が束縛される
    let Polygon { fill, ..} = quadrangle;
    assert_eq!(fill, (0, 0, 0));

    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    let Polygon { mut fill, .. } = polygon;
    fill = (1, 1, 1);
    assert_eq!(fill, (1, 1, 1));
    assert_eq!(polygon.fill, (0, 0, 0));
    polygon.fill = (1, 1, 1);
    assert_eq!(polygon.fill, (1, 1, 1));

    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };

    // 関数型レコードアップデート構文
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)], // vertexesのみ異なる
        .. triangle1 // 他はtriangle1と同じ
    };
}
