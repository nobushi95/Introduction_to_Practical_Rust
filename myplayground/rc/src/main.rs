use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
struct Child(usize);

fn main() {
    let mut rc1 = Rc::new(Child(1)); // Rcポインタ経由でChildをヒープへ格納
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1); // strong_count()で参照カウントを取得
    {
        let rc2 = Rc::clone(&rc1);
        let rc2_ = rc1.clone();
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}, rc2_: {:?}",
            Rc::strong_count(&rc1), rc1, rc2, rc2_
        );
    }
    // rc2のスコープから抜けて、参照カウンタが下がる
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // 参照カウントが1のときは可変の参照が得られる それ以外のときはNone
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1); // downgrade()でWeakポインタが得られる
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1), // 参照カウントは1 Weakポインタはカウントされない
        rc1, weak
    );

    // upgrade()でRc<Child>を得て、Childにアクセス可能
    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1), rc1, rc3
        );
    }

    std::mem::drop(rc1); // dropするとスコープを抜けたのと同じ状態、参照カウントは0になりChildは破棄される
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade()); // upgrade()でアクセス不可
}
