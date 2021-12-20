fn main() {
    let ten = 10;
    let ten_ref = &ten;
     match ten_ref {
         number => assert_eq!(number, &10), // numberは参照（&i32）
     }

     match ten_ref {
         &number => assert_eq!(number, 10), // numberは参照ではない（i32）
     }

     let number = 42;
     let string = match number {
         1 | 2 | 3 => "One or two or three", // パターンの連結
         40..=50 => "From 40 to 50", // 範囲指定
         51..=60 => "From 51 to 60", 
         _ => "Something else",
     };

     println!("{}", string);

     let string = Some("This is a very long string");
     let message = match string {
         Some(s) if s.len() >= 10 => "Long string", // ガード
         Some(_) => "String",
         None => "Nothing",
     };
     println!("{}", message);

     println!("{}", f1(string));
     println!("{}", f1(Some("string")));
     println!("{}", f1(None));

    //  let score = Some(10);
     let score = None;
    //  let score = Some(100);
     if let Some(n) = score {
        println!("{}", n);
     } else {
         println!("None");
     }

     if let Some(100) = score {
         println!("You got full marks!");
     } else {
         println!("You didn't get full marks.");
     }

    //  let score = Some(50);
     let score = None;
     let num = if let Some(n) = score {
         n
     } else {
        0
     };
     println!("{}", num);
}

fn f1(str: Option<&str>) -> &str {
    match str {
        Some(s) if s.len() >= 10 => "Long string",
        Some(_) => "String",
        None => "Nothing",
    }
}
