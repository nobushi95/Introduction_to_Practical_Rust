use toy_vec::ToyVec;

fn main() {
    let e: Option<&String>;
    {
        let mut v = ToyVec::<String>::new();
        v.push("Java Finch".to_string());
        v.push("Bdgerigar".to_string());

        // コンパイルエラー
        // 「参照のライフタイムが値のスコープよりも短いこと」という制約に違反
        // 参照：v.getの戻り値 値：e
        e = v.get(1);
        // getメソッド
        // 引数に&selfが含まれる
        // 戻り値型がOption<&T>で参照型を含む
        // -> getメソッドの戻り値が有効な間はselfは借用中
        // -> 戻り値（e）のライフタイムが引数（self）のライフタイムよりも短くなければならない
    } // vがスコープから抜けるため、v.getで取得できる参照のライフタイムがここで尽きる
    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
