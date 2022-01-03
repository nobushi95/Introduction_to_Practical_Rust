use std::cell::RefCell;

struct A {
    c: char,
    s: String,
}

struct B {
    c: char,
    s: RefCell<String>, // StringをRefCellで包む
}

fn main() {
    let a = A {c: 'a', s: "alex".to_string()};
    let r = &a; // 不変の参照
    // r.s.push('a'); // 不変の参照経由でフィールドを変更しようとするのでエラー

    // これは大丈夫？
    let mut a = A { c: 'a', s: "alex".to_string() };
    let r = &mut a;
    r.s.push('a');
    assert_eq!("alexa", r.s);

    let b = B { c: 'a', s: RefCell::new("alex".to_string()) };
    let rb = &b;
    rb.s.borrow_mut().push('a'); // sに対して可変の参照を取ってpush()
    {
        let rbs = b.s.borrow(); // 不変の参照を取る
        assert_eq!(&*rbs, "alexa");

        // RefCellでは他の参照が有効な間に可変の参照を取ろうとすると実行時にパニックする
        // b.s.borrow_mut();

        // try_borrow_mutならパニックせずにErrを返す
        assert!(b.s.try_borrow_mut().is_err());
    }
    assert!(b.s.try_borrow_mut().is_ok()); // rbsのスコープから抜けるのでOkが変える
}
