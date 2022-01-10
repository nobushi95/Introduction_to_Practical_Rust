use std::string::ToString;

// 引数のimpl Traitは全称impl Trait
fn stringify(t: impl ToString) -> String {
    t.to_string()
}

// 戻り値型のimpl Traitは存在impl Trait
fn to_n(n: i32) -> impl Iterator {
    0..n
}

// impl Traitを使わない場合
use std::ops::Range;
fn to_n_no_impl_trait(n: i32) -> Range<i32> {
    0..n
}

// 存在impl Traitを使う場合
fn to_n_even(n: i32) -> impl Iterator {
    (0..n).filter(|i| i % 2 == 0)
}

// 存在impl Traitを使わない場合
use std::iter::Filter;
fn to_n_even_no_impl_trait(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
    (0..n).filter(|i| i % 2 == 0)
}

use std::fmt;
fn one() -> impl fmt::Display {
    1i32
}

// 戻り値型を匿名化しているだけなので、異なる方を混ぜては返却できない
// fn one_signed(is_signed: bool) -> impl fmt::Display {
//     if is_signed {
//         1i32
//     } else {
//         1u32
//     }
// }

fn gen_counter_trait_object(init: i32) -> Box<dyn FnMut() -> i32> {
    let mut n =init;
    Box::new(move || { // Closure本体にnの所有権を移動する必要がある？
        let ret = n;
        n += 1;
        ret
    })
}

fn gen_counter(init: i32) -> impl FnMut() -> i32 {
    let mut n = init;
    // 存在impl Traitでトレイトオブジェクトを作る必要がなくなった
    move || {
        let ret = n;
        n += 1;
        ret
    }
}

fn main() {
    let n = one();
    // println!("{}", n + n); // i32の情報は失われている（impl Displayのみ返却される）ので+オペレータは使用できない
    let init = 10;
    let mut g = gen_counter(init);
    assert_eq!(g(), init);
    assert_eq!(g(), init + 1);
    assert_eq!(g(), init + 2);
    assert_eq!(g(), init + 3);
    assert_eq!(g(), init + 4);
    
    let init = 20;
    g = gen_counter(init);
    assert_eq!(g(), init);
    assert_eq!(g(), init + 1);
    assert_eq!(g(), init + 2);
    assert_eq!(g(), init + 3);
    assert_eq!(g(), init + 4);
}
