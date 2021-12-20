fn main() {
    // 3, 4, 5は通常i32と推論されるが、u8に型強制される
    let v1: Vec<u8> = vec![3, 4, 5];
    let v1 = vec![3u8, 4u8, 5u8]; // 型強制がなければ、書く必要あり

    // Vec<u8>からスライス&[u8]へ型強制されることで、firstメソッドが使用できる
    assert_eq!(Some(&3u8), v1.first());
    assert_eq!(Some(&3u8), (&v1[..]).first()); // 型強制がなければ、こう書く必要あり

    let mut s1 = String::from("Type coercion");
    let s2 = String::from("is actually easy.");

    // push_str()はpush_str(self: &mut String, s: &str)
    // s1がmut Stringから&mut Stringへ型強制
    // &s2が&Stringから&strへ型強制
    s1.push_str(&s2);
    (&mut s1).push_str(s2.as_str()); // 型強制がなければ、こう書く必要あり

    let mut i1 = 0u8;
    i1 = 255; // 255は255u8ではないが、u8へ型強制される

    let p1 = &&&&[1, 2, 3, 4];
    let p2: &[i32; 4] = p1; // 型強制が&&&&a1 -> &&&a1 -> &&a1 -> &a1 の順に推移的に作用する

    let p3 = &&[1, 2, 3, 4];
    // &&[i32; 4]からスライス&[i32]まで段階を踏むと型強制可能
    let p4: &[i32; 4] = p3;
    let p5: &[i32] = p4;
    // 一度に&&[i32; 4]からスライス&[i32]まで型強制できない
    // let p6: &[i32] = p3;
}
