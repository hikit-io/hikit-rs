use hi_push::client;
use hi_push::service::model::RegisterTokenParams;

#[tokio::test]
async fn feature_email() {
    // let client_id = std::env::var("EMAIL_CLIENT_ID").unwrap();
    // let client_secret = std::env::var("EMAIL_CLIENT_SECRET").unwrap();

    let cli = client::Client::new(
        "http://127.0.0.1:4523/m1/2238314-0-default/",
        "",
        "smtp.office365.com",
    );
    let resp = cli
        .register_token(RegisterTokenParams {
            group: "".to_string(),
            token: "".to_string(),
            ch_id: "".to_string(),
            _override: None,
        })
        .await;
    println!("{:?}", resp);
}
