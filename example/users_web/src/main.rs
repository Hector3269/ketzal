use ketzal::config::Bootstrap;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    Bootstrap::default()
        .create()
        .await
}
