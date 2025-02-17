use anyhow::Error;

#[tokio::test]
async fn quick_dev() -> Result<(), Error> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Bogdan").await?.print().await?;
    Ok(())
}