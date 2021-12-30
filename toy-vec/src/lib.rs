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
    // indexはusize型なので、コピーされる
    // Option<&T>を返すので、selfが所有する値の不変の参照を返却する
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    // 関数の戻り値型が参照型のとき、コンパイラは以下の規則に基づいて「ライフタイムの省略」を試みる
    // 1. 引数のうち、参照型が1つだけなら、その引数から借用する
    // 2. 第1引数が&self、&mut selfなら、他に参照型の引数があっても、selfから借用する
    // 3. 上記以外の場合はライフタイムを省略できない
    // get_or関数では、3に該当するため、指定する必要がある
    // デフォルトでは"get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T"と推論され、
    // 戻り値は'aを期待するがdefautlは'bとなり、ライフタイムが一致しないのでエラー
    // 以下のようにライフタイムを同一にすると、戻り値が有効な間はselfとdefaultの両方が借用中となる
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
    }

    // whereを使ってライフタイムを指定することもできる
    // pub fn get_or<'a, 'b>(&'a self, index: usize, default: &'b T) -> &'a T 
    // where
    //     'b: 'a // 'b は'aよりライフタイムが長い
    // {}

    fn grow(&mut self) {

    }

    // 戻り値が参照ではないので、所有権ごと返す
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // &mut selfでToyVec（elements）を借用しているので、
            // 以下のコードは借用の文脈で値がムーブアウトしていることになる
            // 借用経由では値の所有権を一方的に奪うことはできないのでエラー
            // let elem = self.elements[self.len]; 
            // 以下のように値を変わりの値と交換することで解決（replaceの戻り値は置換前の値）
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }
}