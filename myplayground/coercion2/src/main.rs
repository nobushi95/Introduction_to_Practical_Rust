fn  f1(n: &mut usize, str: &str, slice: &[i32]) {
    *n =str.len() + slice.len();
}

fn f2(slice: &[usize]) -> usize {
    slice.len()
}

fn f3(slice: &mut [usize]) {
    let len = f2(slice); // ポインタの弱体化により&mut [usize]から&[usize]へ型強制
    slice[0] = len;
}

fn f4(p: &[i32]) -> i32 { p[0] }
fn f5(p: Box<[i32]>) -> i32 { p[0] }

fn main() {
    let mut b1 = Box::new(10);
    let s1 = String::from("deref");
    let v1 = vec![1, 2, 3];

    // Derefによる型強制
    // &b1：&mut Box<usize> -> &mut usize
    // &s1：&String -> &str
    // &v1：&Vec<i32> -> &[i32]
    f1(&mut b1, &s1, &v1);
    assert_eq!(8, *b1);

    let mut v = vec![0; 10];
    f3(&mut v[..]);
    assert_eq!(10, v[0]);

    let a1 = [1, 2, 3, 4];
    assert_eq!(f4(&a1), 1); // &[i32; 4] -> &[i32]
    assert_eq!(f5(Box::new(a1)), 1); // Box[i32; 4] -> Box<[i32]>

    let mut d: Box<dyn std::fmt::Debug>;
    // Debugトレイトを実装する型はトレイトオブジェクトへ型強制できる
    d = Box::new([1, 2]); // Box<[i32; 2]> -> Box<Debug>
    d = Box::new(Some(1)); // Box<Some<i32>> -> Box<Debug>

    let v1: Vec<u8> = vec![3, 4, 5];
    // firstメソッドのレシーバはVec<u8>だが、Vecにはfirstメソッドは定義されていない
    // レシーバが&[u8]へと暗黙的に型強制されることで、スライスに定義されたfirstメソッドを呼べる
    // 1.Derefによる型強制：Vec<u8> -> [u8]
    // 2.レシーバの参照化（autorefd：automatically referenced）：[u8] -> &[u8]
    let v1_1 = v1.first();
}
