use actix_web::{HttpResponse, Json, Query, State};
use failure::Error;
use log::debug;

use crate::Server;

pub fn hanle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    // POSTされたファイルはActix Webでは簡単に扱えないのでここではまだコードなし

    // レスポンスはDefaultでダミーデータを生成
    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

pub fn handle_post_logs(
    server: State<Server>,
    // POSTのボディはJson<T>を引数に書くと自動的にデシリアライズされる
    log: Json<api::logs::post::Request>,
) -> Result<HttpResponse, Error> {
    // Json<T>はTへのDeferを実装しているので内部ではそのままTの値として扱える
    debug!("{:?}", log);
    // レスポンスはAccepted
    Ok(HttpResponse::Accepted().finish())
}

pub fn handle_get_logs(
    server: State<Server>,
    // クエリパラメターはQuery<T>を引数に書くと自動的にデシリアライズされる
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

pub fn handle_get_csv(
    server: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);
    // CSVファイルはバイナリデータにして返す
    let csv: Vec<u8> = vec![];
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
