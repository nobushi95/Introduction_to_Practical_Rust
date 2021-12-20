fn main() {
    let mut one = 1;
    // moveキーワードにより、クロージャに変数の値をコピー
    // moveキーワードは変数の所有権をクロージャに移転するが、
    // Copyトレイトを実装するものは、クロージャに値がコピーされる
    let plus_one = move |x| {
        x + one
    };

    one += 1;
    println!("plus_one(10): {}", plus_one(10));
    println!("one: {}", one);
}
