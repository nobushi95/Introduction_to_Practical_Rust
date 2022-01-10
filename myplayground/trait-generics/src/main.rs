trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

trait As<T> {
    fn cast(self) -> T;
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

impl As<u32> for i8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

fn main() {
    let data = Box::init("foo");
    println!("data: {:?}", data);
    println!("*data: {:?}", *data);
    assert_eq!(data, Box::new("foo"));
    assert_eq!(*data, "foo");
    let d1 = data.clone(); // clone（ムーブしない）
    let d2 = *data;
    let d3 = data;
    // let d4 = data; // dataはd3にムーブしているのでエラー

    // トレイトのジェネリクス型を明示するには型名::<型>と記述する
    let data = Box::<f32>::init(0.1);
    // 型推論が働く場合は、トレイト名でもかける
    let data: Box<f32> = Init::init(0.1);
    let data: Box<_> = Init::<f32>::init(0.1);

    let one_u32: u32 = 1u8.cast();
    let one_u64: u64 = 1.cast(); // 1はu8
    let one_u32: u32 = 1i8.cast();
    // let one_i32: i32 = 1.cast(); // As<i32>の実装がないためエラー
}
