type UserName = String;

#[derive(Debug)]
// #[derive(Default)]による自動導出はできない
enum Task {
    Open,
    AssignedTo(UserName), // タプル構造体
    // フィールド構造体
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

impl Default for Task {
    fn default() -> Self {
        Open // use crate::Task::*; によりOpenのみで記載可能
    }
}

use crate::Task::*;

fn main() {
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
        Open,
        Default::default(),
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされています", i, assignee);
            }
            Working { assignee, remaining_hours } => {
                println!("タスク{}は{}さんが作業中です。残り{}時間の見込み", i, assignee, remaining_hours);
            }
            _ => {
                println!("タスク{}はその他のステータス（{:?}）です", i, task);
            }
        }
    }
}
