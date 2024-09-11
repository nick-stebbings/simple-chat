use server::run;
mod command;
mod connection;
mod user;
mod user_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}
