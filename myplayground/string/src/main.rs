fn main() {
    /* 文字列の連結 */ 
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");
    let s3 = "ストロベリー".to_owned(); // かつてはto_string()よりもto_owned()が推奨されていた

    s1.push_str("タルト"); // String型に&str型の文字列を追加
    println!("s1: {}", s1);

    s2.push('と'); // String型にcharを追加
    println!("s2: {}", s2);

    // s2.push_str(s3); // push_str()が受け取るのは&str型のみ
    s2.push_str(&s3); // &をつけると型強制により&Stringから&strへと変換される
    println!("s2: {}", s2);

    /* to_string() */
    let f = 4.3 + 0.1;
    println!("f.to_string(): {}", f.to_string());
    println!("format!(\":.2\", f): {}", format!("{:.2}", f));
    
    let t = (1, "ABC");
    println!("format!(\":?\", t): {}", format!("{:?}", t)); // tupleはDebugトレイトを実装しているのでfomat!マクロで変換できる

    /* parse() */
    let s1 = "42"; // シャドーイングにより前に定義されている変数を隠ぺいする
    println!("s1.parse::<i32>(): {:?}", s1.parse::<i32>());

    let s2 = "abc";
    let r2 : Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("r2: {:?}", r2);

    /* char配列から文字列へ変換 */
    let cs = ['t', 'r', 'u', 's', 't'];
    let s1 = cs.iter().collect::<String>(); // String型へは変換可
    println!("s1: {}", s1);
    // let s1 = cs.iter().collect::<str>(); // str型へは変換不可
    let s2 = cs[1..].iter().collect::<String>();
    println!("s2: {}", s2);

    let bad_utf8 = [
        b'a',             // a
        0xf0, 0x90, 0x80, // でたらめなバイト列
        0xe3, 0x81, 0x82, // あ
    ];
    let s = String::from_utf8_lossy(&bad_utf8); // 不正な値を含むバイト列からもStringを作れる
    println!("s: {}", s);

    // utf16バイト文字列からStringへの変換
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];
    if let Ok(s) = String::from_utf16(&utf16) {
        println!("s: {}", s);
    } else {
        unreachable!();
    }

    // 文字列からu8配列（バイト文字配列）の作成
    let bs1 = b"abc\xe3\x81\x82";
    println!("bs1: {:?}", bs1);
    let bs2 = br#"ab\ncd"#;
    println!("bs2: {:?}", bs2);
}
