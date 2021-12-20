fn main() {
    let a1 = [1; 100];
    for i in 0..100 {
        assert_eq!(a1[i], 1);
    }

    // 以下のコードはコンパイル時にsizeの値がきまらないのでエラー
    // let size = 100;
    // let a2 = [false; size];

    let a3 = ["Hello"; 3];
    for s in a3.iter() {
        println!("{}", *s);
    }
}
