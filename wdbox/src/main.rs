use actix_web::web::Data;
use chrono::Local;
use clap::Parser;
use log::{info, warn};
use std::io::Write;
use wdbox::server::start_http_server;
use wdbox::{Commands, Passwd};

use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                record.level(),
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.args()
            )
        })
        .init();
    let cli = Passwd::parse();
    info!("命令行参数:{:?}", cli);
    let app_data = Data::new(Arc::new(cli.clone()));
    let result = match cli.commands {
        Commands::Decode => {
            info!("文件解密");
            cli.decode_tofile()
        }
        Commands::Encode => {
            info!("文件加密");
            cli.encode_tofile()
        }
        #[allow(unused_variables)]
        Commands::Service { ref log_path } => cli.service(),

        #[allow(unused_variables)]
        Commands::StartServer { address, port } => {
            let command = app_data.clone();
            tokio::spawn(async move {
                loop {
                    info!("执行自动加密");
                    let _ = command.encode_tofile();
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                }
            });
            info!("通讯服务中");
            start_http_server(app_data).await
        }
    };
    if result.is_err() {
        warn!("运行时错误:{}", result.err().unwrap());
    }
    Ok(())
}
