use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;
    client.set("Tanaka", "Maiko".into()).await?;

    let result = client.get("Tanaka").await?;

    println!("get value from the server; result={:?}", result);

    Ok(())
}

// async fn say_hello() {
//     println!("world");
// }

// #[tokio::main]
// async fn main() {
//     let op = say_hello();

//     println!("hello");

//     op.await;
// }