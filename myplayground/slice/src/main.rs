fn print_info(name: &str, sl: &[char]) {
    println!("    {:10} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),
        sl.first(),
        sl[1],
        sl.last()
    )
}

fn main() {
    let a =['a', 'b', 'c', 'd'];
    println!("a: {:?}", a);

    print_info("&a[..]",    &a[..]);
    print_info("&a",        &a);
    print_info("&a[1..3]",  &a[1..3]);
    print_info("&a[1..=2]", &a[1..=2]);

    let v = vec!['e', 'f', 'g', 'h'];
    println!("v: {:?}", v);

    print_info("&v[..]",    &v[..]);
    print_info("&v",        &v);
    print_info("&v[1..3]",  &v[1..3]);
    print_info("&v[1..=2]", &v[1..=2]);

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4]; // immutable slice
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"one"));
    assert_eq!(s3[1], "two");
    assert_eq!(s3.get(1), Some(&"two"));
    assert_eq!(s3.get(3), None);
    assert!(s3.contains(&"two"));
    assert!(s3.starts_with(&["one", "two"]));
    assert!(s3.ends_with(&["two", "three"]));

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];
    a4[2..6].sort();
    // a4.sort();
    println!("a4: {:?}", a4);
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    let (s4a, s4b) = a4.split_at_mut(5);
    println!("s4a: {:?}", s4a);
    println!("s4b: {:?}", s4b);
    
    s4a.reverse();
    println!("s4a: {:?}", s4a);

    s4b.sort_unstable();
    println!("s4b: {:?}", s4b);
}
