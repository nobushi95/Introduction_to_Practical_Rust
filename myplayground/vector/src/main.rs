fn main() {
    let mut v6 = vec!['a', 'b', 'c', 'd', 'e'];
    println!("v6: {:?}", v6);
    v6.push('d');
    v6.push('e');
    println!("v6: {:?}", v6);
    println!("v6.pop(): {:?}", v6.pop());
    println!("v6: {:?}", v6);
    v6.insert(1, 'f');
    println!("v6: {:?}", v6);
    println!("v6.remove(2): {}", v6.remove(2));
    println!("v6: {:?}", v6);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    println!("v6: {:?}", v6);
    println!("v7: {:?}", v7);

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);
    println!("v6: {:?}", v6);
    println!("a8: {:?}", a8);
}
