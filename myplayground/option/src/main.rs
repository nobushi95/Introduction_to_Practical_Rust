fn main() {
    let mut o2 = Some(String::from("Hello"));
    println!("o2.unwrap(): {}", o2.unwrap());

    o2 = None;
    let o2_unwraped = o2.unwrap_or_else(|| String::from("o2 is None"));
    println!("o2_unwraped: {}", o2_unwraped);

    let mut o3 = Some(25);
    println!("o3.map(|n| n * 10): {:?}", o3.map(|n| n * 10));
    println!("o3: {:?}", o3);

    o3 = None;
    println!("o3.map(|n| n * 10): {:?}", o3.map(|n| n * 10));
    println!("o3: {:?}", o3);

    o3 = Some(10);
    let o3_mapped_and_then = o3.map(|n| n * 10).and_then(|n| if n >= 200 { Some(n) } else { None });
    println!("o3_mapped_and_then: {:?}", o3_mapped_and_then);
    println!("o3: {:?}", o3);

    let a1 = [3, 7, 31, 127];
    println!("add_elems(a1): {:?}", add_elems(&a1));
    println!("add_elems(a1[..=1]): {:?}", add_elems(&a1[..=1]));
}

fn add_elems(s: &[i32]) -> Option<i32> {
    // ?演算子
    // Some(値)なら値を取り出し、Noneならこの関数からすぐに戻り、Noneを返す
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}
