use anyhow::Error;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<(), Error> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Bogdan").await?.print().await?;
    hc.do_get("/hello2/Bogdan").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc
        .do_post(
            "/api/login",
            json!(
                {
                    "username": "Bogdan",
                    "password": "123"
                }
            ),
        )
        .await?;
    req_login.print().await?;

    let req_login = hc
        .do_post(
            "/api/login",
            json!(
                {
                    "username": "demo1",
                    "password": "welcome"
                }
            ),
        )
        .await?;
    req_login.print().await?;

    Ok(())
}
