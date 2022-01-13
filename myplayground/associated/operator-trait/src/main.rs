use core::ops::Add;
use core::ops::AddAssign;

struct MyInt(i64);

impl Add<MyInt> for MyInt {
    type Output = Self;
    fn add(self, rhs: MyInt) -> Self::Output {
        MyInt(self.0 + rhs.0)
    }
}

impl AddAssign for MyInt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

fn main() {
    let one = MyInt(1);
    let two = MyInt(2);
    let mut i = one + two;
    i += MyInt(3);
    assert_eq!(6, i.0);
}