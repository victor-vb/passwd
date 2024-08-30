use std::{io::ErrorKind, sync::Arc};

use crate::{Host, Passwd};
use actix_web::{
    error::PayloadError,
    get,
    http::Error,
    web::{Data, Query},
    App, HttpServer,
};
use log::info;

#[get("/passwd")]
pub async fn passwd(app_data: Data<Arc<Passwd>>, query: Query<Host>) -> Result<String, Error> {
    info!("host:{:?}", query);
    let encode_passwds = app_data
        .get_passwd(&query.host, None)
        .map_err(|e| PayloadError::from(std::io::Error::new(ErrorKind::Other, e)))?;
    return Ok(encode_passwds);
}

pub async fn start_http_server(app_data: Data<Arc<Passwd>>) -> Result<bool, String> {
    let address = app_data.get_socket();
    let _ = HttpServer::new(move || App::new().app_data(Data::clone(&app_data)).service(passwd))
        .bind(address)
        .map_err(|e| e.to_string())?
        .run()
        .await;
    Ok(true)
}
