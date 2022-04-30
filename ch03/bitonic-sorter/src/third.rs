use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    match *order {
        SortOrder::Ascending  => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}

use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
where
    F: Fn(&T, &T) -> Ordering
{
    // if is_power_of_two(x.len()) {
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The lenght of x is not a power of two. (x.len(): {})", x.len()))
    }
}

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

// cargo test 実行時のみコンパイルされる
#[cfg(test)]
mod tests {
    // use super::{is_power_of_two, sort, sort_by};
    use super::{sort, sort_by};
    use crate::SortOrder::*;

    // cargo test 実行時のみ実行
    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }

    // deriveアトリビュートを使い、DebugトレイトとPartialEqトレイトの実装を自動抽出
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name:  String,
        age: u8,
    }

    impl Student {
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                // to_stringメソッドで&str型の引数からString型を得る
                first_name: first_name.to_string(),
                last_name:  last_name.to_string(),
                age, // フィールドと変数が同じ名前のときは、省略可能
            }
        }
    }

    // impl PartialEq for Student {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.first_name == other.first_name &&
    //         self.last_name  == other.last_name  &&
    //         self.age        == other.age
    //     }
    // }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro    = Student::new("Taro",    "Yamada", 16);
        let hanako  = Student::new("Hanako",  "Yamada", 14);
        let kyoko   = Student::new("Kyoko",   "Ito",    15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];
        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro    = Student::new("Taro",    "Yamada", 16);
        let hanako  = Student::new("Hanako",  "Yamada", 14);
        let kyoko   = Student::new("Kyoko",   "Ito",    15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];
        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];
        assert_eq!(sort_by(&mut x,
            &|a, b| a.last_name.cmp(&b.last_name)            // まずlast_nameを比較する -> last_nameが等しくなければ（Less or Greater）、それを返す
            .then_with(|| a.first_name.cmp(&b.first_name))), // last_nameが等しい（Equal）ならfirst_nameを比較する
            Ok(())
        );
        assert_eq!(x, expected);
    }

    use crate::utils::{new_u32_vec, is_sorted};
    #[test]
    fn sort_u32_large() {
        let vec_size = 65536;
        {
            let mut x = new_u32_vec(vec_size);
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // assert!(is_sorted_ascending(&x));
            assert!(is_sorted(&x, &Ascending));
        }
        {
            let mut x = new_u32_vec(vec_size);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            // assert!(is_sorted_descending(&x));
            assert!(is_sorted(&x, &Descending));
        }
    }
}