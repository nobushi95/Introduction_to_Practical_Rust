/// 位置情報 .0から.1までの区間を表す
/// ex. Loc(4, 6)：5文字目から7文字目を表す（0始まり）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

/// アノテーション　値にLocをもたせたもの
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self {value, loc}
    }
}
fn main() {
    println!("Hello, world!");
}
