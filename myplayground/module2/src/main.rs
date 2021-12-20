use rand::prelude::*;

mod network {
    pub fn ping() {
        println!("ping");
    }
}

fn main() {
    // ここはルートモジュール内
    network::ping();
    // crateはクレートのルートモジュールを指す
    crate::network::ping();
    // selfは現在のモジュールを指す
    self::network::ping();
    // superは現在のモジュールの親モジュールを指す（ここはルートモジュールなので親モジュールはない）
}
