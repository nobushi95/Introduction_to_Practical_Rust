// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

// implブロック内に、トレイト境界としてDefaultを設定
impl<T: Default> ToyVec<T> {

    // キャパシティが0のToyVec
    pub fn new() -> Self {
        Self::width_capacity(0)
    }

    // キャパシティが指定サイズのToyVec
    pub fn width_capacity(capcacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capcacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) //T型のデフォルト値をsize個作り
            .collect::<Vec<_>>()// Vec<T>に収集
            .into_boxed_slice() // Box<[T]>に変換
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    // &mut self を取るので、ToyVecの内容は変更される
    // elemtnはT型なので、所有権がこのメソッドへ移動する（最終的にはToyVecに移動する）
    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element; // elementの所有権が移動
        self.len += 1;
    }

    // &selfを取るので、ToyVecの内容は変更されない
    // Option<&T>を返すので、selfが所有する値の不変の参照を返却する
    // indexはusize型なので、コピーされる
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {

    }
}