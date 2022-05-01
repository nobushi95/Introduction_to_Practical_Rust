//! wordcountはシンプルな文字、単語、行の出現頻度の計算機能を提供します。
//! 詳しくは[`count`]関数のドキュメントを見てください

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// [`count`]で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字毎に頻度を数える
    Char,
    /// 単語毎に頻度を数える
    Word,
    /// 行毎に頻度を数える
    Line,
}

/// オプションのデフォルトは[`Word`](CountOption::Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// `input`から1行ずつUTF-8文字列を読み込み、頻度を数える
///
/// 頻度を数えるオプションによって聖書される.
/// * [`CountOption::Char`] : Unicodeの1文字毎に頻度を数える
/// * [`CountOption::Word`] : 正規表現 `\w+` にマッチする単語枚に頻度を数える
/// * [`CountOption::Line`] : `\n`または`\r\n`で区切られた1行毎に頻度を数える
///
/// # Examples
/// 入力中の単語の頻度を数える例
///
/// ```
/// use std::io::Cursour;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
///```
///
/// # Panics
///
/// 入力がUTF-8でフォーマットされていない場合にパニックする
// ライブラリ外から参照するためにpubにする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
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
