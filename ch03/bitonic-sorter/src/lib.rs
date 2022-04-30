// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub mod first;
pub mod second;
pub mod third;
pub mod utils;

pub mod fourth;

pub enum SortOrder {
    Ascending,  // 昇順
    Descending, // 降順
}