use std::{env, error::Error, net::SocketAddr};

use axum::{extract::State, response::Html, routing::get, Router};

use sqlx::{Pool, Postgres};
use state::AppState;

mod folder;
mod note;
mod state;
mod tag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_host = env::var("APP_HOST").expect("env.APP_HOST not found");
    let app_port = env::var("APP_PORT").expect("env.APP_PORT not found");

    let addr = format!("{}:{}", app_host, app_port)
        .parse::<SocketAddr>()
        .expect(&format!(
            "Faild to create a socket addr from {}:{}",
            app_host, app_port
        ));

    let app_state = get_state().await;
    let routes = get_routes(app_state);

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn migrate_db(pool: &Pool<Postgres>) {
    sqlx::migrate!("db/migrations")
        .run(pool)
        .await
        .expect("Database migration failed");
}

async fn get_db_pool() -> Pool<Postgres> {
    let db_host = env::var("DB_HOST").expect("env.DB_USER not found");
    let db_port = env::var("DB_PORT").expect("env.DB_PORT not found");
    let db_name = env::var("DB_NAME").expect("env.DB_NAME not found");
    let db_user = env::var("DB_USER").expect("env.DB_USER not found");
    let db_pass = env::var("DB_PASSWORD").expect("env.DB_PASSWORD not found");

    let db_url =
        format!("postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}");

    sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to the database")
}

async fn get_state() -> AppState {
    let pool = get_db_pool().await;
    AppState { pool }
}

fn get_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(|State(_): State<AppState>| async { Html("hello world") }),
        )
        .merge(folder::routes::routes())
        .merge(tag::routes::routes())
        .merge(note::routes::routes())
        .with_state(app_state)
}

#[cfg(test)]
mod test {
    use hyper::{self, Body, Request};
    use std::{net::TcpListener, time::Duration};
    use testcontainers::{
        clients::Cli, core::WaitFor, images::generic::GenericImage, Container,
    };

    use super::*;

    pub async fn start<'a>(docker: &'a Cli) {
        let container = start_postgres(docker);
        let db_port = container.get_host_port_ipv4(5432);
        let conn_str =
            format!("postgres://root:root@0.0.0.0:{}/notepad", db_port);

        let pool = sqlx::postgres::PgPool::connect(&conn_str)
            .await
            .expect("Unable to connect to the database");

        start_service(AppState { pool }).await;
    }

    pub async fn start_service(app_state: AppState) {
        let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(get_routes(app_state).into_make_service())
                .await
                .unwrap();
        });
    }

    pub fn start_postgres<'a>(docker: &'a Cli) -> Container<'a, GenericImage> {
        let image = GenericImage::new("postgres", "latest")
            .with_env_var("POSTGRES_PASSWORD", "root")
            .with_env_var("POSTGRES_USER", "root")
            .with_env_var("POSTGRES_DB", "notepad")
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            // for some reason eventhough container prints "database is ready"
            // connection fails and adding 1 sec fixes it atlease on this pc
            .with_wait_for(WaitFor::Duration {
                length: Duration::from_secs(1),
            });

        docker.run(image)
    }

    #[tokio::test]
    async fn service_is_running() {
        // start all the services
        let docker: Cli = testcontainers::clients::Cli::docker();
        start(&docker).await;

        let client = hyper::client::Client::new();
        let response = client
            .request(
                Request::builder()
                    .uri("http://0.0.0.0:3000")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

        assert_eq!(std::str::from_utf8(&body[..]).unwrap(), "hello world");

        ()
    }
}
