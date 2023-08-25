use axum::extract::State;
use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;

use axum::response::Html;

use crate::config::Config;

mod config;

async fn hello_world(State(config): State<Config>) -> Html<String> {
    Html(format!(
        "
            <h1>Hello, world!</h1>
            <h3>Secrets</h3>
            <ul>
                <li>SECRET_1: {}</li>
                <li>SECRET_2: {}</li>
                <li>NOT_A_SECRET: {}</li>
            </ul>
    ",
        &config.secret_1, &config.secret_2, &config.not_a_secret
    ))
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let config = Config::try_from(secret_store)?;

    let router = Router::new()
        .route("/", get(hello_world))
        .with_state(config);

    Ok(router.into())
}
