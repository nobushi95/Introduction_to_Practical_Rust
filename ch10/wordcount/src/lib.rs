use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoutOption {
    Char,
    Word,
    Line,
}

impl Default for CoutOption {
    fn default() -> Self {
        CoutOption::Word
    }
}

// ライブラリ外から参照するためにpubにする
pub fn count(input: impl BufRead, option: CoutOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CoutOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    // 分割した単語を文字列としてHashMapへ追加
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}
