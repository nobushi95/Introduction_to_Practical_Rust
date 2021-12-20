fn main() {
    println!("add1(\"3\", \"4\"): {:?}", add1("3", "4"));
    println!("add1(\"3\", \"abc\"): {:?}", add1("3", "abc"));
}

fn add1(s0: &str, s1: &str) -> Result<i32, String> {
    let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
    Ok(s0 + s1)
}
