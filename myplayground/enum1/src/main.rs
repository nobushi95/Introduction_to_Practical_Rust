// Debugトレイト："{:?}"で表示できるようになる
// PartialEqトレイト：==演算子が使えるようになる
#[derive(Debug, PartialEq)]
enum Weekday {
    Monday, Tuesday, Wednesday, Thursday, Friday,
}

fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGIF!"); // Thank God, it's Friday やっと金曜日だ
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Friday);
    assert_eq!(3, Month::March as isize);
}

// データがない列挙型ではisize型の整数値を割り当てられる
enum Month {
    January = 1,
    Feburary = 2,
    March = 3,
    April = 4,
    May = 5, 
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
