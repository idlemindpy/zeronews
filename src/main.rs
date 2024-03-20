use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero::configuration::get_configuration;
use zero::startup::run;
use zero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy(&config.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
