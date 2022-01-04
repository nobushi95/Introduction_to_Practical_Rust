use lazy_static::lazy_static;
use std::collections::HashSet;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    // staticな変数DOGSを導入する
    // この変数はプログラム全体で共有される
    // refは束縛モードと呼ばれ、不変の参照を意味する
    // lazy_staticではDOGSが初めて参照外しされたときに以下の初期化コードが実行される
    pub static ref DOGS: RwLock<HashSet<&'static str>> = {
        // 内側のミュータビリティ実現のために、HashSetをRwLockで包む
        let dogs = ["柴", "トイプードル"].iter().cloned().collect();
        RwLock::new(dogs)
    };
}
fn main() -> Result<(), Box<dyn Error>> {
    {
        let dogs = DOGS.read()?; // readロックを取得
        assert!(dogs.contains("柴"));
        assert!(dogs.contains("トイプードル"));
    } // dogsがスコープから外れてreadロック解除

    fn stringify(x: impl ToString) -> String { x.to_string() }

    // writeロックを取得して、HashSetに要素を追加
    DOGS.write()?.insert("ブル・テリア");

    // DOGSはstaticな変数なので、moveがあってもなくてもよい？（所有権は考えなくて良い？）
    std::thread::spawn(|| {
        // 別スレッドでwriteロックを取得してHashSetに要素を追加
        DOGS.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    }).join().expect("Thread error")?;

    // mainスレッドと別スレッドの両方で追加した要素が見える
    // DOGSは'staticライフタイムを持つので、Err(PoisonError(ガード))がmain関数から返せるので、map_str(stringify)は不要
    assert!(DOGS.read()?.contains("ブル・テリア"));
    assert!(DOGS.read()?.contains("コーギー"));

    Ok(())
}
