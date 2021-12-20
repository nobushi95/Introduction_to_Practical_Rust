mod module {
    // 構造体や列挙型の可視性はデフォルトでprivate
    #[derive(Default)]
    struct Polygon1 {
        vertexes: Vec<(i32, i32)>,
        stroke_width: u8,
        fill: (u8, u8, u8),
        inteernal_id: String,
    }

    #[derive(Default)]
    pub struct Polygon2 {
        pub vertexes: Vec<(i32, i32)>,
        pub stroke_width: u8,
        pub fill: (u8, u8, u8),
        inteernal_id: String, // このフィールドのみprivate
    }

    type UserName = String;
    enum enum1 {
        Open,
        AssignedTo(UserName),
        Working {
            assignee: UserName,
            remainig_hours: u16,
        }
    }

    // 列挙型は列挙型自身の可視性のみ制御可能
    // バリアントやバリアントの持つフィールドの可視性は制御できない
    // 列挙型をpubにすると、バリアントやバリアントのフィールドもpubとなる
    pub enum enum2 {
        Open,
        AssignedTo(UserName),
        Working {
            assignee: UserName,
            remainig_hours: u16,
        }
    }
}

// use shape::Polygon1; // エラー
use module::Polygon2;
// use module::enum1; // エラー
use module::enum2;

fn main() {
}