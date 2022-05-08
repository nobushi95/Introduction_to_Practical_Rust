use chrono::{DateTime, Utc};
use serde_derive::*;

// JSONの{"user_agent": "xxx", "response_time": 0, "timestamp": "yyyy-MM-dd+HH:mm:ss"}に対応
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Log {
    pub user_agetnt: String,
    pub response_time: i32,
    // 戻り値で使うログはtimestampがOptionではない
    pub timestamp: DateTime<Utc>,
}

// クエリパラメータの `?from=yyyy-MM-dd+HH:mm:ss&until=yyyy-MM-dd+HH:mm:ss` に対応
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

pub mod csv {
    pub mod get {
        use crate::DateTimeRange;
        pub type Query = DateTimeRange;
        // getはファイルを返すのでResopnse型の定義は不要
    }

    pub mod post {
        use serde_derive::*;

        // CSVファイルを受け付けるのでリクエストデータはない
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        // 受領したログの数を返す
        pub struct Response(pub usize);
    }
}

pub mod logs {
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde_derive::*;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        // 保存しているログをすべて返す
        pub struct Response(pub Vec<Log>);
    }

    pub mod post {
        use chrono::{DateTime, Utc};
        use serde_derive::*;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Request {
            pub user_agent: String,
            pub response_time: i32,
            pub timestamp: Option<DateTime<Utc>>,
        }
        // Acceptedを返すのでResponseデータ型は不要
    }
}
