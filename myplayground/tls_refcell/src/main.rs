// TLS(Thread Local Storage)
// スレッド固有のデータを格納できるメモリ領域で、スレッド終了時に破棄される
// TLSを使用するとスレッド内でのみ有効なグローバル変数のようなものが実現できる
// TLSに格納した値は外向きには不変となるため、RefCellのような内側のミュータビリティを実現するものでデータを包む必要がある

use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    // TLSに変数RABBITSを作成する
    // thread_localマクロはmutキーワードをサポートしない
    thread_local! {
        static RABBITS: RefCell<HashSet<&'static str>> = {
            // 初期化はそのスレッドで初めてここにアクセスしたときに実行される
            // 2要素のHashSetを作成し、RefCellで包む
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    };

    // TLSにおいた値にアクセスするにはwithを使う
    // ここではmainスレッドのRABBITSが得られる
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    std::thread::spawn(|| // 別スレッドを起動し、要素を追加
        RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト"))
    ).join().expect("Thread error"); // スレッドの修了を待つ

    // mainスレッドのRABBITSへアクセスする
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト")); // 別スレッドで追加した要素は存在しない（RABBITSはスレッドごとに持つ）
    })
}
