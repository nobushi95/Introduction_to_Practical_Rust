fn main() {
    static I0: i32 = 42;
    let mut s0: &'static str;
    let s1 = "42"; // &str型（静的領域）
    let s2 = 42.to_string(); // String型（ヒープ領域）
    s0 = s1;
    // s0 = &s2; コンパイルエラー

    take_static(s1);
    // take_static(&s2); // &String 'static要求を満たせないのでコンパイルエラー
    take_static(s2); // String 参照ではないので渡せる
    // Stringは明確には'staticスコープを持たないが、関数が所有している値は保持ている限り開放されないので、
    // 借用規則を考えなくても良い => ライフタイムを追跡しなくてもメモリ安全性が保証されるのでコンパイル可能
}

fn take_static<T: 'static>(_x: T) {}
