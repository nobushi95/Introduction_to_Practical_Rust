use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

use crate::SortOrder;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // let mut v = Vec::with_capacity(n);
    // for _ in 0..n {
    //     v.push(rng.sample(&Standard));
    // }
    // v

    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)はもとのイテレータから最初のn要素だけを取り出すイテレータを返す
    // collect()はイテレータから値を収集して、コレクタ（コンテナ？）に格納する
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows(2)はもとのイテレータから1要素刻みで２要素ずつ値を取り出す新しいイテレータを返す。
    // [1, 2, 3, 4] -> [1, 2], [2, 3], [3, 4]

    // all()はイテレータから値を取り出し、クロージャに渡す
    // クロージャがfalseを返せば、そこで終了し、falseを返す
    // クロージャがtrueを返している間は、イテレータから次の値を取り出しクロージャへ渡し続ける
    // イテレータの値が尽きるまで（Noneになるまで）一度もfalseを返さなければ、all()はtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}

pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool {
    let comparator = match *order {
        SortOrder::Ascending  => |pair: &[T]| pair[0] <= pair[1],
        SortOrder::Descending => |pair: &[T]| pair[0] >= pair[1],
    };
    x.windows(2).all(comparator)
}