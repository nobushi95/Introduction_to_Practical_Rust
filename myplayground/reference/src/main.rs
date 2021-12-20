fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

fn main() {
    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0; // i32型
    let n1_ptr = &mut n1; // &mut i32型。可変の参照（ミュータブルな参照）
    assert_eq!(*n1_ptr, 0);

    // 可変の参照では参照先の値を変更できる
    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1000);

    // 型注釈がないと、関数定義型と推論される
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(32), 64);

    // fに型注釈がないとエラー
    f = abs;
}
