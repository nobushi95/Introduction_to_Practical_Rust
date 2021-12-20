// 記法1
// mod network;

// 記法2
// mod network;
// use network::server;

// 記法3
mod network;
use network::server::*;

fn main() {
    // 記法1
    // network::server::echo();

    // 記法2
    // server::echo();

    // 記法3
    echo();
}
