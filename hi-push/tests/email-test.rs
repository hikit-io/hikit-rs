use hi_push::{email, Pusher};

#[tokio::test]
async fn feature_email() {
    let client_id = std::env::var("EMAIL_CLIENT_ID").unwrap();
    let client_secret = std::env::var("EMAIL_CLIENT_SECRET").unwrap();

    let cli = email::Client::new(&client_id, &client_secret, "smtp.office365.com").await;
    let to = vec!["public@hikit.io", "1234"];
    let msg = email::Message {
        title: "Hello Email",
        body: "This is test.",
        to: to.as_slice(),
    };
    let res = cli.retry_push(&msg).await;
    println!("{:?}", res);
}
