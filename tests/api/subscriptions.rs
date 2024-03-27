use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // returns 200 when form input for /subscribe is valid

    let app = spawn_app().await;
    let body = "name=le%20chien&email=cedar%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_persists_subscriber() {
    let app = spawn_app().await;
    let body = "name=le%20chien&email=cedar%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "cedar@gmail.com");
    assert_eq!(saved.name, "le chien");
    assert_eq!(saved.status, "pending");
}

#[tokio::test]
async fn subscribe_invalid() {
    // return 400 when form input for /subscribe is invalid

    let app = spawn_app().await;
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
async fn subscribe_sends_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=le%20chien&email=cedar%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    // Mock asserts on drop
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_link() {
    let app = spawn_app().await;
    let body = "name=le%20chien&email=cedar%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn present_but_empty_fields() {
    let app = spawn_app().await;
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
