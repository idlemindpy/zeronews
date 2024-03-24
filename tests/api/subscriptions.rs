use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_valid() {
    // returns 200 when form input for /subscribe is valid

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20chien&email=cedar%40gmail.com";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "cedar@gmail.com");
    assert_eq!(saved.name, "le chien");
}

#[tokio::test]
async fn subscribe_invalid() {
    // return 400 when form input for /subscribe is invalid

    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20chien", "missing email address"),
        ("email=cedar%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with error 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn present_but_empty_fields() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=cedar%40gmail.com", "empty name"),
        ("name=le%20chien&email=", "empty email"),
        ("name=&email=deffos-no-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 OK when payload was {}",
            description
        );
    }
}
