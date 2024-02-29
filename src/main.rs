use crate::configuration::get_configuration;
use crate::startup::run;
use std::net::TcpListener;

mod configuration;
mod routes;
mod startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
