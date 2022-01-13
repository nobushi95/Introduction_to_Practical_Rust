use std::{io, path::Path, fs::File};

fn take_string_string(_s: String) {}

fn take_string_str(s: &str) {
    let _s = s.to_string(); // 呼び出し元ではStringから&strに変換し、内部でStringへ戻しているので、コストが2重
}

fn take_string(s: impl Into<String>) {
    let _s = s.into();
}

fn range(_min: impl Into<Option<usize>>, _max: impl Into<Option<usize>>) {}

// パスネームとして振る舞うものが複数ある Path、PathBuf、String、OsStr、OsString
// 上記はすべて、AsRef<Path>を実装している
fn hello_to_file(path: impl AsRef<Path>) -> io::Result<()> {
    // let mut file = File::new(path.as_ref())?; // File::newがない？？
    // write!(file, "Hello, File")?;
    Ok(())
}

trait SomeTrait {
    fn take_ref(&self);
}

impl SomeTrait for str {
    fn take_ref(&self) {}
}

#[derive(Debug)]
enum Either<TypeA, TypeB> {
    A(TypeA),
    B(TypeB),
}

use std::fmt;
impl<TypeA, TypeB> fmt::Display for Either<TypeA, TypeB>
where
    TypeA: fmt::Display,
    TypeB: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Either::A(a) => a.fmt(f),
            Either::B(b) => b.fmt(f),
        }
    }
}

fn main() {
    take_string_string("some_string".to_string()); // 文字列リテラルは渡せないのでto_stringを呼ぶ必要がある

    let arg = "string".to_string();
    take_string_str(arg.as_str()); // String型からas_str()で&strへ変換
    
    // 文字列リテラルもStringも渡せる
    take_string("some_string");
    take_string(arg); // arg.as_str(&self)はなので、所有権を移動していない ここでムーブ

    // 片方に値を渡し、片方に渡さない場合を記述できる
    // 値を渡すときにSome(1)としなくて良い
    range(1, None);

    let s = "hoge";
    s.take_ref(); // take_ref(&self)なので、&strに対しても呼び出しできる

    // let box_s = Box::new(*s); // コンパイル時にSizeがわからない？
    // box_s.take_ref();

    // 登場する型をEnumで把握していれば、トレイトオブジェクトを使わなくても混合できる
    // boolとi32を混ぜてvecに挿入できる
    let mut v: Vec<Either<bool, i32>> = vec![];
    v.push(Either::A(true));
    v.push(Either::B(1i32));
    for e in v {
        println!("{:?}: {}", e, e);
    }
}
