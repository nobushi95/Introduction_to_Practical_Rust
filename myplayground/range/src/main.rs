use std::fmt::Debug;
use std::slice::SliceIndex;

fn print_range(array: &[char], range: std::ops::Range<usize>) {
    print!("[ ");
    for idx in range {
        if let Some(n) = array.get(idx) {
            print!("{} ", n);
        }
    }
    println!("]");
}

fn range(start: usize, end: usize) -> std::ops::Range<usize> {
    start..end
}

fn print_range2<T1, T2>(array: &[T1], range: T2)
where
    T2: std::ops::RangeBounds<usize> + SliceIndex<[T1]>,
    <T2 as SliceIndex<[T1]>>::Output: Debug,
{
    let tmp = &array[range];
    println!("{:?}", tmp);
}

fn print_slice<T>(name: &str, array: &[T])
where
    T: Debug,
{
    println!("{}: {:?}", name, array);
}

fn main() {
    let a = ['a', 'b', 'c', 'd', 'e'];

    let a1 = &a[..];
    print_slice("a1", a1);
    let a2 = &a[..3];
    print_slice("a2", a2);
    let a3 = &a[..=3];
    print_slice("a3", a3);
    let a4 = &a[1..];
    print_slice("a4", a4);
    let a5 = &a[1..3];
    print_slice("a5", a5);
    let a6 = &a[1..=3];
    print_slice("a6", a6);

    print_range2(&a, ..);
}
