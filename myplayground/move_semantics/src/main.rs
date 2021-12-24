use std::ops::Drop;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn f0(p: Parent) {
    println!("p: {:?}", p);
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent) {
    p.0 += 10;
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 値の所有権がp1からp2に移動
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // p1には所有権がないため、アクセス不能

    f0(p2); // コピートレイトを実装していないのでコピーが行われず、値の所有権がp2からf0の引数pに移動
    // println!("p2: {:?}", p2); // p2には所有権がないため、アクセス不能

    let mut p3 = Parent(1, Child(11), Child(12));
    f1(&p3); // p3は借用される
    f2(&mut p3); // p3は借用される
    println!("p3: {:?}", p3); // p3は所有権を失っていないので、アクセス可能
}
