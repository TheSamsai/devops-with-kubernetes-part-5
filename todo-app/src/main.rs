use std::{net::SocketAddr, time::Instant, sync::Arc};

use axum::{
    routing::{get, get_service, put, post},
    Router, response::{IntoResponse, Html, Redirect}, http::StatusCode, Extension, extract::{Form, Path}, Json,
};

use nats::Connection;
use serde::Deserialize;
use tower_http::services::ServeFile;

use tokio::{process::Command, sync::Mutex};

use tera::Tera;
use tera::Context;

use sqlx::{postgres::PgPoolOptions, PgPool};

type ImageAge = Arc<Mutex<Instant>>;
type ImageStorage = Arc<String>;

mod models;

use models::Todo;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").map(|val| val.parse::<u16>().unwrap()).unwrap_or(3000);
    let image_storage = Arc::new(std::env::var("IMAGE_DIR").unwrap_or(String::from("./image")));

    let image_age: ImageAge = Arc::new(Mutex::new(Instant::now()));
    download_image_of_the_day(image_storage.clone()).await;

    let postgres_password = std::env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password"));
    let postgres_host = std::env::var("POSTGRES").unwrap_or(String::from("localhost/todos"));

    let postgres = format!("postgres://postgres:{}@{}", postgres_password, postgres_host);

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres).await.expect("Failed to connect to Postgres!");

    initialize_database(&pool).await;

    let nats_url = std::env::var("NATS_URL").unwrap_or(String::from("nats://localhost:4222"));
    let nats_client = nats::connect(nats_url).expect("Failed to connect to NATS");

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let image_storage_path: String = image_storage.to_string();

    let app = Router::new()
        .route("/image", get_service(ServeFile::new(format!("{}/image.jpg", image_storage_path))).handle_error(handle_error))
        .route("/todos", get(get_todos).post(post_todo).put(put_todo))
        .route("/mark/:id", post(toggle_todo_status))
        .route("/health", get(health_check))
        .route("/", get(index_page).post(post_todo_form))
        .layer(Extension(image_storage))
        .layer(Extension(image_age))
        .layer(Extension(pool))
        .layer(Extension(nats_client))
        .layer(Extension(tera));

    let addr = SocketAddr::from(([0,0,0,0], port));

    println!("Started at port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn initialize_database(pool: &PgPool) {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS todos
(
    id   BIGSERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    done BOOLEAN DEFAULT FALSE
);
        "#
    )
    .execute(pool)
    .await.expect("Failed to create database table");
}

async fn index_page(
    Extension(image_age_state): Extension<ImageAge>,
    Extension(image_storage): Extension<ImageStorage>,
    Extension(pool): Extension<PgPool>,
    Extension(tera): Extension<Tera>
) -> Html<String> {
    check_and_download_image_of_the_day(image_age_state, image_storage).await;

    let todos = Todo::get_all(&pool).await;

    let mut context = Context::new();

    context.insert("todos", &todos);

    Html(tera.render("index.html", &context).unwrap())
}

#[derive(Deserialize, Debug)]
struct TodoInput {
    todo: String
}

async fn post_todo_form(
    Form(input): Form<TodoInput>,
    Extension(pool): Extension<PgPool>,
    Extension(nats_client): Extension<Connection>
) -> Redirect {
    let todo = Todo { id: 0, text: input.todo, done: false };

    if todo.text.len() > 140 {
        println!("Todo too long: {}", todo.text);

        return Redirect::to("/");
    }

    todo.create(&pool).await;

    nats_client.publish("todo", "Todo was created");

    println!("Todo added via form");

    Redirect::to("/")
}

async fn toggle_todo_status(
    Path(todo_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Extension(nats_client): Extension<Connection>
) -> Redirect {
    if let Some(mut todo) = Todo::get_by_id(&pool, todo_id).await {
        todo.done = !todo.done;
        todo.update(&pool).await;

        nats_client.publish("todo", "Todo was updated");
    }

    println!("Todo status toggled via form");

    Redirect::to("/")
}

async fn get_todos(
    Extension(pool): Extension<PgPool>
) -> Json<Vec<Todo>> {
    let mut todos = Todo::get_all(&pool).await;

    return Json(todos);
}

async fn post_todo(
    Json(payload): Json<TodoInput>,
    Extension(pool): Extension<PgPool>,
    Extension(nats_client): Extension<Connection>
) -> Json<Vec<Todo>> {
    let todo = Todo { id: 0, text: payload.todo, done: false};

    if todo.text.len() > 140 {
        println!("Todo too long: {}", todo.text);

        return Json(Todo::get_all(&pool).await);
    }

    todo.create(&pool).await;

    nats_client.publish("todo", "Todo was created");

    println!("Todo added via POST /todos");

    return Json(Todo::get_all(&pool).await);
}

async fn put_todo(
    Json(todo): Json<Todo>,
    Extension(pool): Extension<PgPool>,
    Extension(nats_client): Extension<Connection>
) -> Json<Vec<Todo>> {
    if todo.text.len() > 140 {
        println!("Todo too long: {}", todo.text);

        return Json(Todo::get_all(&pool).await);
    }

    todo.update(&pool).await;

    nats_client.publish("todo", "Todo was updated");

    println!("Todo updated via PUT /todos");

    return Json(Todo::get_all(&pool).await);
}

async fn check_and_download_image_of_the_day(image_age: ImageAge, image_storage: ImageStorage) {
    let mut image_age = image_age.lock().await;

    if Instant::now().duration_since(*image_age).as_secs() > 24 * 60 * 60 {
        println!("Updating image of the day");
        download_image_of_the_day(image_storage).await;
        *image_age = Instant::now();
    }
}

async fn download_image_of_the_day(image_dir: Arc<String>) {
    Command::new("wget")
        .arg("https://picsum.photos/1200")
        .arg("-O")
        .arg(format!("{}/image.jpg", image_dir))
        .spawn()
        .expect("Failed to start 'wget'")
        .wait()
        .await
        .expect("'wget' failed to run");
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

// This functions as both a liveness and readiness check
// because the program will keep crashing until database is ready
async fn health_check(
    Extension(pool): Extension<PgPool>
) -> Result<String, StatusCode> {
    if pool.is_closed() {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Ok("Alive".to_string())
    }
}
