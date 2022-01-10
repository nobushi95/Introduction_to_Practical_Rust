// trait Overload {
//     fn call(&self) -> &'static str;
// }

// impl Overload for i32 {
//     fn call(&self) -> &'static str {
//         "i32"
//     }
// }

// impl Overload for str {
//     fn call(&self) -> &'static str {
//         "str"
//     }
// }

trait Overload1<T> {
    fn call(&self, _: T) -> &'static str;
}

impl Overload1<i32> for i32 {
    fn call(&self, _: i32) -> &'static str {
        "(i32, i32)"
    }
}

impl Overload1<char> for i32 {
    fn call(&self, _: char) -> &'static str {
        "(char, i32)"
    }
}

fn main() {
    // Overload1を実装してしまうと、callの候補が複数あってエラー(Overloadのみの場合はOK)となるので、コメントアウト
    // Rustのメソッド呼び出しは糖衣構文
    // assert_eq!(1i32.call(), "i32");
    // assert_eq!("str".call(), "str");
    // // 糖衣構文を使用しないと以下のようになる
    // assert_eq!(Overload::call(&1i32), "i32"); // 明示的にOverloadを使用するので、callが一意に決まる
    // assert_eq!(Overload::call("str"), "str");

    assert_eq!(1i32.call(2i32), "(i32, i32)");
    assert_eq!(1i32.call('a'), "(char, i32)");
}
