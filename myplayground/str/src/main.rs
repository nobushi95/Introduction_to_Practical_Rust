fn main() {
    let s3 = "文字列を複数行に渡って書くと
    開業やスペースが入る";
    println!{"s3: {}", s3};
    let s4 = "行末にバックスラッシュをつけると\
    改行などが入らない";
    println!{"s4: {}", s4};

    let s5 = "文字列に\"と\\を含める";
    println!{"s5: {}", s5};
    let s6 = r#"文字列に"と\を含める"#;
    println!{"s6: {}", s6};

    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    println!{"s7: {}", s7};

    let s8 = "もちろん絵文字\u{1f600}も使える";
    println!{"s8: {}", s8};

    let fruits = "あかりんご, あおりんご\nラズベリー、ブラックベリー";
    // lines()メソッドは改行コードを含む文字列から1行ずつ取り出せるイテレータを作る
    let mut lines = fruits.lines();
    let apple_line = lines.next();
    let apple_line_str = apple_line.unwrap();
    println!("apple_line: {:?}", apple_line);
    println!("apple_line_str: {}", apple_line_str);
    println!("lines.next: {:?}", lines.next());
    println!("lines.next: {:?}", lines.next());

    if let Some(apples) = apple_line {
        println!("apples: {}", apples);
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        println!("apples.find(\"あお\"): {:?}", apples.find("あお"));

        let mut apple_iter = apples.split(",");
        println!("apple_iter.next(): {:?}", apple_iter.next());
        
        let green = apple_iter.next();
        println!("green: {:?}", green);
        println!("green.map(str::trim): {:?}", green.map(str::trim));
        println!("apple_iter.next(): {:?}", apple_iter.next());
    } else {
        unreachable!();
    }

    println!();
    println!();

    let s ="かか\u{3099}く";
    println!("{}", s);
    let mut iter = s.chars();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let utf8:[u8; 4]= [0x61, 0xe3, 0x81, 0x82];
    println!("std::str::from_utf8(&utf8): {:?}", std::str::from_utf8(&utf8));

    let bad_utf8: [u8; 2] = [0x81, 0x33];
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("result2: {:?}", result2);

    println!();
    println!();

    let mut s1 = "abcあいう".to_string();
    let s2 = s1.as_mut_str();

    s2.make_ascii_uppercase(); // 英小文字を英大文字へ
    println!("s2: {}", s2);

    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';
    println!("s1: {}", s1);
    println!("\"あ\".len(): {}", "あ".len());
}
