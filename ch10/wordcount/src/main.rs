use std::env;
use std::fs::File;
use std::io::BufReader;

// libクレートに分離したものを使う
use wordcount::count;

fn main() {
    // コマンドラインの1つめに指定された引数をを読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
