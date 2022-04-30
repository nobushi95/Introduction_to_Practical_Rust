use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        // 行を単語で分割
        for m in re.find_iter(&line) {
            // 分割した単語を文字列としてHashMapへ追加
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
        }
    }

    freqs
}

fn main() {
    // コマンドラインの1つめに指定された引数をを読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader);
    println!("{:?}", freqs);
}
