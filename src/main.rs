use zero2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =  TcpListener::bind("127.0.0.1:8000").expect("Failed to bind port 8000");
                
    run(listener)?.await
}
