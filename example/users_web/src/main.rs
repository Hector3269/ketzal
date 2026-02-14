use ketzal::config::Bootstrap;

mod app;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Bootstrap::default().create().await
}
