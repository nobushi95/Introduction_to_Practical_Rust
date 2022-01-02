use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());

    let mut iter = v.iter(); 

    // 借用規則2：値が共有されている間（不変の参照が有効な間）は値の変更を許さない に違反する
    // v.push("Hill Mynah".to_string()); // v.push()は可変の参照を必要とする
    // v.iter()で借用後に、iter.next()で再度借用しようとするのでコンパイルエラー
    // Iter構造体のelementsのライフタイムを生成するToyVec構造体のライフタイムと同じにしたため

    assert_eq!(iter.next(), Some(&"Java Finch".to_string())); // iter.next()で再度借用
    v.push("Canary".to_string());
}