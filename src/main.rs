use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connectionto the mini-redis address.
    //
    // The client::connect function is provided by the mini-redis crate It
    // asynchronously establishes a TCP connection with the specified remote
    // address. Once the connection is established, a client handle is returned.
    // Even though the operation is performed asynchronously, the code we write
    // looks synchronous. The only indication that the operation is asynchronous is
    // the .await operator.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
