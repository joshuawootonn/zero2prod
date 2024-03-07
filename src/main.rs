use crate::configuration::get_configuration;
use crate::startup::run;
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

mod configuration;
mod routes;
mod startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect ot Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    run(listener, connection_pool)?.await
}
