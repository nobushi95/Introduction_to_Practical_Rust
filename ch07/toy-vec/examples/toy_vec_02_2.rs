use toy_vec::ToyVec;

fn main() {
    let mut v: ToyVec<usize> = ToyVec::new();
    v.push(100);
    let e = v.get(0); // 不変の参照へのアクセス（借用）
    assert_eq!(e, Some(&100));
    v.push(200); // ベクタを変更する
    assert_eq!(e, Some(&100)); // ここで再度借用しようとするのでコンパイルエラー
}