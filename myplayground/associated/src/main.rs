// 関連関数: selfを引数に取らない関数（トレイト・データ型）
// 関連定数: トレイト・データ型に関連付けられた定数

trait Trait {
    const CONST_T_NUM: u32 = 0; // トレイトの関連定数（デフォルトも可能？）
    fn from_str_a(str_a: &StrA) -> Self; // トレイトの関連関数
}

struct StrA {
    m: f64,
}

impl StrA {
    const CONST_A_NUM: u32 = 1; // トレイトの関連定数
    fn new() -> Self { // データ型の関連関数
        StrA {
            m: 0.0, 
        }
    }
}

struct StrB {
    m: i32
}

impl Trait for StrB {
    const CONST_T_NUM: u32 = 2;
    fn from_str_a(str_a: &StrA) -> Self {
        StrB {
            m: str_a.m as i32,
        }
    }
}

// 関連型: トレイトにのみ使用できる機能で、トレイトに関連づいた型を定義できる
use std::str::FromStr;

trait Server {
    // type 型名 で宣言できる
    type Response;
    // type 型名: トレイト境界 で宣言できる
    type Request: FromStr;

    // 関連型は Self::型名 でアクセスする
    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;
impl Server for EchoServer {
    // type 型名 = 型名 で定義する
    type Response = String;
    type Request = String;

    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

// S::Responseのように関連型を参照できる
// 関連型については特別指定しなければ、任意関連型を受け付ける
// fn handle<S: Server>(server: S, req: &str) -> S::Response {
//     // 関連型にトレイト境界がついているのでトレイトの関数を呼び出すこともできる
//     let req = S::Request::from_str(&req).unwrap(); // unwrap()でコンパイルエラー？
//     server.handle(req)
// }

// 関連型が特定の型を持っていることを指定する場合
// トレイト名<関連形名 = 型名> で指定できる
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

trait Generics<T> {}
trait AssociatedType {
    type T;
}

// trait Generics<T> {
//     fn new(t: T) -> Self;
// }

// trait AssociatedType {
//     type T;
//     fn new(t: Self::T) -> Self;
// }

// ジェネリクスは引数の型が必要
fn some_fun_gen<S, T: Generics<S>>(t: T) {}
// 関連型は推論できるなら引数の型は省略可能
fn some_fun_asc<T: AssociatedType>(t: T) {}

// トレイト境界で特定の型を指定する場合
fn some_fun_gen_u32<T: Generics<u32>>(t: T) {}
fn some_fun_asc_u32<T: AssociatedType<T = u32>>(t: T) {}

struct Baz;

impl<T> Generics<T> for Baz {}

// Tが使用していないのでエラー
// impl<T> AssociatedType for Baz {}

struct Qux<T> {
    x: T,
}

// Tが使用されるのでOK
impl<T> AssociatedType for Qux<T> {
    type T = T;
}

struct Fiz;

// ジェネリクスは1対多
impl Generics<i32> for Fiz {}
impl Generics<char> for Fiz {}

// 関連型1対1
impl AssociatedType for Fiz {type T = i32;}
// 関連型は2つ目以降はエラー
// impl AssociatedType for Fiz {type T = char;}

// 関連型は外部からも参照できる
fn get_from_gen<D: Generics<VeryLong<Type, Name>>>(d: D) -> (VeryLong<Type, Name>, VeryLong<Type, Name>) {}
fn get_from_asc<D: AssociatedType<T = VeryLong<Type, Name>>>(d: D) -> (D::T, D::T) {}

fn main() {

}