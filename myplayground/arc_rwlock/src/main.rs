use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

// ?演算子を使うためmain関数からResult型を返す
// PoisonErrorをそのまま返そうとするとガードがデータへの参照を持っているためライフタイムエラーとなりコンパイルできない
// map_err(stringify)とすることでPoisonErrorのto_stringメソッドを呼び、
// PoisonError(ガード)からエラー文字表現に変換することで、参照がなくなりmain関数から返せるようになる
fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    // HashSetの内側のミュータビリティを実現するためにRwLockで包んで、スレッドで共有するためにArcで包む
    let dogs =Arc::new(RwLock::new(dogs));
    // RwLockが内容するデータへはガードと呼ばれるデータ構造でアクセスする

    // xはToStringトレイトを実装していることを意味する？
    fn stringify(x: impl ToString) -> String { x.to_string() }

    {
        // readロックを取得する（ArcがDerefトレイトを実装しているため、RwLockのreadを使える）
        // let ds = dogs.read().map_err(|x| x.to_string());
        // map_errでガードが取得できるとOk(ガード)が取得できるので?演算子でアンラップ
        let ds = dogs.read().map_err(stringify)?;
        // ガードがDerefトレイトを実装しているため、HashSetのcontainsを使える
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    } // dsがスコープから外れ、readロック解除

    // writeロックを取得し、HashSetの要素を追加
    dogs.write().map_err(stringify)?.insert("ブル・デリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move || { // moveでキャプチャした変数（dogs1）の所有権を移動
        // 別スレッドでwriteロックを取得しHashSetに要素を追加（ArcがDerefトレイトを実装しているため、RwLockのwriteを使える）
        // ガードがDerefトレイトを実装しているため、HashSetのinsertを使える
        dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    }).join().expect("Thread error")?; // スレッドの修了を待つ

    // PoisonErrorを発生させる
    {
        let dogs1 = Arc::clone(&dogs);
        std::thread::spawn(move || {
            let _guard = dogs1.write();
            panic!();
        }).join().expect_err(""); // スレッドの修了を待つ（Errが帰ってくるのを期待）

        dogs.read().expect("Cannnot acquire read lock");
    }

    // mainスレッドと別スレッドの両方で追加した要素が見える（ガードがDerefトレイトを実装しているため、HashSetのcontainsを使える）
    assert!(dogs.read().map_err(stringify)?.contains("ブル・デリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));

    Ok(())
}