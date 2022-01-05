fn apply_fn<F>(f: &F, ch: char)
where
    F: Fn(char) -> bool,
{
    assert!(f(ch));
}

fn apply_fn_mut<F>(f: &mut F, ch: char)
where
    F: FnMut(char) -> bool,
{
    assert!(f(ch));
}

fn apply_fn_once<F>(f: F, ch: char)
where
    F: FnOnce(char) -> bool,
{
    assert!(f(ch));
}

fn main() {
    let s1 = "read-only";
    let mut lookup = |ch| s1.find(ch).is_some(); // find(&self)は&strを読むだけ
    apply_fn(&lookup, 'r');
    apply_fn_mut(&mut lookup, 'o'); // Fnを実装するクロージャはFnMutも実装する
    apply_fn_once(lookup, 'y'); // Fnを実装するクロージャはFnOnceも実装する
    assert_eq!(s1, "read-only"); // 環境に取り込まれた文字列（&str）は変更されない

    let mut s2 = "append".to_string();
    let mut modify = |ch| {
        s2.push(ch); // push(&mut self, char)はStringを可変の参照経由で変更する
        true
    };
    // apply_fn(&modify, 'e'); // Fnをとる関数はコンパイルエラー（modifyはFnMutを実装する）
    apply_fn_mut(&mut modify, 'e'); // FnMutを取る関数はOK
    apply_fn_once(modify, 'd'); // FnMutを実装するクロージャはFnMutも実装する
    assert_eq!(s2, "appended"); // &strとStringの比較はできる？

    let s3 = "be converted".to_string();
    let mut consume = |ch| {
        let bytes = s3.into_bytes(); // into_bytes(self)はStringの所有権を奪う
        bytes.contains(&(ch as u8))
    };
    // apply_fn(&consume, 'b'); // Fnを取る関数はエラー
    // apply_fn_mut(&mut consume, 'c'); // FnMutを取る関数もエラー
    apply_fn_once(consume, 'c'); // FnOnceを取る関数のみOK
    // assert_eq!(s3, "error"); // s3はムーブ済みなのでエラー
}
