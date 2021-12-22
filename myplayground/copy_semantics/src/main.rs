// Copyトレイとの実装条件
// 1. その型とすべてのフィールドがCopyトレイトを実装している
// 2. その型とすべてのフィールドがデストラクタ（Dropトレイト）を実装していない
// 3. その型がColenトレイトを実装している
#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child, Child);

#[derive(Copy, Clone, Debug)]
struct Child(usize);

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // p1からp2へコピー
    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1);
}
