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

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 値の所有権がp1からp2に移動
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // p1には所有権がないため、アクセス不能
}
