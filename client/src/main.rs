#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    index().await
}

async fn index() -> Result<(), reqwest::Error>{
    let res = reqwest::get("http://127.0.0.1:8080/").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}