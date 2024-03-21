use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero::configuration::get_configuration;
use zero::startup::run;
use zero::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());
    let address = format!("{}:{}", config.app.host, config.app.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
