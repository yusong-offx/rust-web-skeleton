use crate::route::{make_service, AppState};

use super::config::InnerConfig;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

impl InnerConfig {
    // pub fn logger_init(&self) {
    //     let subscriber = FmtSubscriber::builder()
    //         .with_max_level(match self.log.level.as_str() {
    //             "trace" => Level::TRACE,
    //             "debug" => Level::DEBUG,
    //             "info" => Level::INFO,
    //             "warn" => Level::WARN,
    //             "error" => Level::ERROR,
    //             _ => Level::INFO,
    //         })
    //         .finish();
    //     tracing::subscriber::set_global_default(subscriber).unwrap();
    // }

    // pub async fn database_init(&self) -> Surreal<Client> {
    //     let db = Surreal::new::<Ws>(format!("{}:{}", self.database.host, self.database.port))
    //         .await
    //         .unwrap();
    //     db.use_ns(self.database.dbnamespace.as_str())
    //         .use_db(self.database.dbname.as_str())
    //         .await
    //         .unwrap();
    //     db.signin(Root {
    //         username: self.database.user.as_str(),
    //         password: self.database.password.as_str(),
    //     })
    //     .await
    //     .unwrap();
    //     db
    // }

    pub async fn http_init(&self, state: AppState) -> tokio::task::JoinHandle<()> {
        let http_service = make_service(state);
        let http_bind =
            tokio::net::TcpListener::bind(format!("{}:{}", self.http.host, self.http.port))
                .await
                .unwrap();
        let http_server: tokio::task::JoinHandle<()> = tokio::spawn(async move {
            axum::serve(http_bind, http_service).await.unwrap();
        });
        http_server
    }

    // TODO : grpc server start...
    #[allow(dead_code)]
    pub fn grpc_init(&self) -> tokio::task::JoinHandle<()> {
        unimplemented!("grpc server start...");
    }
}
