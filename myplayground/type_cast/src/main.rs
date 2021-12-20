fn main() {
    let i1 = 42;
    let f1 = i1 as f64;
    let c1 = 'a';
    assert_eq!(c1 as u32, 97);

    let i2 = 300;
    let u1 = i2 as u8;
    assert_eq!(u1, 44); // asではオーバーフローチェックは行われない

    let t1 = ('a', 42);
    let t2 = (t1.0 as u32, t1.1 as u8); // タプルは1要素ずつキャストする櫃ようあり

    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    let v2 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>(); // イテレータにマップ（型変換）を適用してコレクト
    let v3: Vec<u8> = From::from("hello"); // Fromトレイトによる変換
    assert_eq!(v1, v3);
}
