use std::string::ToString;
use std::fmt::Display;

fn stringify(t: impl ToString) -> String {
    t.to_string()
}

fn stringify_i32(t: i32) -> String {
    <i32 as ToString>::to_string(&t)
}

fn stringify_dyn_box(t: Box<dyn ToString>) -> String {
    t.to_string()
}

fn stringify_dyn(t: &dyn ToString) -> String {
    t.to_string()
}

fn main() {
    // 静的ディスパッチ
    stringify(1i32);
    // 上のメソッドは下のメソッドのように展開されるイメージ
    stringify_i32(1);
    assert_eq!(stringify_i32(1), "1".to_string());

    // 動的ディスパッチを使用するにはトレイトオブジェクトを使う
    // トレイトオブジェクトはdyn Traitで記述する
    let mut v: Vec<&dyn Display> = vec![];
    // Displayトレイトを実装するboolとi32をvに格納
    v.push(&true);
    v.push(&1i32);

    // 動的ディスパッチ
    assert_eq!(stringify_dyn_box(Box::new(10)), "10".to_string());
    assert_eq!(stringify_dyn(&11), "11".to_string());
}
