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
    // sqlx::migrate!("db/migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Database migration failed");

    let host = env::var("HOST").expect("env.HOST not found");

    let addr = host
        .parse::<SocketAddr>()
        .unwrap_or(SocketAddr::from(([0, 0, 0, 0], 3000)));

    let routes = routes().await;

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn db_pool() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL").expect("env.DATABASE_URL not found");

    sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Unable to connect to the database")
}

async fn state() -> AppState {
    let pool = db_pool().await;
    AppState { pool }
}

#[allow(dead_code)]
async fn routes() -> Router {
    let app_state = state().await;

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
    use std::{collections::HashMap, net::TcpListener};
    use testcontainers::{core::WaitFor, Image};

    use super::*;

    pub async fn start_service() {
        let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes().await.into_make_service())
                .await
                .unwrap();
        });
    }

    pub struct Postgres {
        env_vars: HashMap<String, String>,
    }

    impl Default for Postgres {
        fn default() -> Self {
            return Self {
                env_vars: HashMap::from([(
                    String::from("POSTGRES_PASSWORD"),
                    String::from("root"),
                )]),
            };
        }
    }

    impl Image for Postgres {
        type Args = ();

        fn name(&self) -> String {
            String::from("postgres")
        }

        fn tag(&self) -> String {
            String::from("latest")
        }

        fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
            vec![WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            )]
        }

        fn env_vars(
            &self,
        ) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
            Box::new(self.env_vars.iter())
        }
    }

    #[tokio::test]
    async fn service_is_running() {
        start_service().await;

        let docker = testcontainers::clients::Cli::docker();
        docker.run(Postgres::default());

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
