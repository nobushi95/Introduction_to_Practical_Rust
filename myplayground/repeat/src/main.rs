fn main() {
    let mut counter = 0;
    let ten = loop {
        if counter == 10 {
            break counter;
        }
        counter += 1;
    };
    println!("ten: {}", ten);

    // let mut counter = Some(0);
    let mut counter = Some(7);
    // let mut counter = None;
    repeat(&mut counter);
    println!("counter: {:?}", counter);
}

fn repeat(counter: &mut Option<i32>) {
    while let Some(i) = *counter {
        if i == 10 {
            *counter = None;
        } else {
            println!("{}", i);
            *counter = Some(i + 1);
        }
    }
}
