#[derive(Default)]
struct A {
    f0: u8,
    f1: u32,
    f2: u8,
}

#[repr(C)]
#[derive(Default)]
struct B {
    f0: u8,
    f1: u32,
    f2: u8,
}

#[allow(unused)]
// フィールドに参照を持つ構造体
// ライフタイム指定子（'a）が必要
struct StrRefs<'a> {
    s1: &'a str,
    s2: &'a str,
}

fn main() {
    let a: A = Default::default();
    println!(
        "struct A ({} bytes)\n  f0: {:p}\n  f1: {:p}\n  f2: {:p}\n",
        std::mem::size_of::<A>(),
        &a.f0,
        &a.f1,
        &a.f2
    );
    let b: B = Default::default();
    println!(
        "struct B ({} bytes)\n  f0: {:p}\n  f1: {:p}\n  f2: {:p}\n",
        std::mem::size_of::<B>(),
        &b.f0,
        &b.f1,
        &b.f2
    );
}
