use std::net::TcpListener;

mod routes;
mod startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");

    startup::run(listener)?.await
}
