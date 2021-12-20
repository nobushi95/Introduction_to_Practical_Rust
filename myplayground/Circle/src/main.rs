struct Circle {
    radius: u32,
}

impl Circle {
    // レシーバー：メソッド呼び出し時の受け手となる値、OOPにおけるオブジェクト
    // メソッドの第1引数はメソッドレシーバー
    // &self：メソッドのレシーバー（構造体のインスタンス）を、イミュータブルな参照として使用可能
    // &mut self：メソッドのレシーバー（構造体のインスタンス）を、ミュータブルな参照として使用可能
    // self：メソッドのレシーバー（構造体のインスタンス）の所有権がメソッドに移動
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // 関連関数：構造体型そのもの（インスタンスではない）に関連付けられた関数
    // 第1引数はselfにしない
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}

fn main() {
    let circle1 = Circle { radius: 10 };
    println!("Circle1's diameter: {}", circle1.diameter());
    let circle2 = Circle::small_circle();
    println!("Circle2's diameter: {}", circle2.diameter());
}
