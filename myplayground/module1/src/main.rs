mod server {
    pub(crate) fn echo() { // crate内でのみpublic
        println!("server");
    }
}

mod client {
    // pub(in app::network) fn echo() { // app::networkモジュール内でのみpublic
    pub fn echo() {
        println!("client");
    }
}

fn main() {
    server::echo();
    client::echo();
}
