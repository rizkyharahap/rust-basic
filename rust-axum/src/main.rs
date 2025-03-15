use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use axum::{
    Extension, Form, Json, Router,
    body::{Body, Bytes},
    error_handling::HandleError,
    extract::{Multipart, Path, Query, Request, State, rejection::JsonRejection},
    middleware::{Next, from_fn, map_request},
    response::{IntoResponse, Response},
    routing::{get, post},
    serve,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn test_axum() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server = TestServer::new(app).unwrap();
    let response = server.get("/").await;

    response.assert_status_ok();
    response.assert_text("Hello, World!");
}

#[tokio::test]
async fn test_method_routing() {
    async fn hello_world() -> String {
        "Hello, World!".to_string()
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello, World!");

    let response = server.post("/post").await;
    response.assert_status_ok();
    response.assert_text("Hello, World!");
}

#[tokio::test]
async fn test_request() {
    async fn hello_world(request: Request) -> String {
        format!("Hello {}", request.method())
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");

    let response = server.post("/post").await;
    response.assert_status_ok();
    response.assert_text("Hello POST");
}

#[tokio::test]
async fn test_uri() {
    async fn hello_world(uri: Uri, method: Method) -> String {
        format!("Hello {} {}", method, uri.path())
    }

    let app = Router::new()
        .route("/get", get(hello_world))
        .route("/post", post(hello_world));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET /get");

    let response = server.post("/post").await;
    response.assert_status_ok();
    response.assert_text("Hello POST /post");
}

#[tokio::test]
async fn test_query() {
    async fn route(Query(params): Query<HashMap<String, String>>) -> String {
        format!("Hello {}", params.get("name").unwrap())
    }

    let app = Router::new().route("/query", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/query").add_query_param("name", "Rizki").await;
    response.assert_status_ok();
    response.assert_text("Hello Rizki");
}

#[tokio::test]
async fn test_header() {
    async fn route(headers: HeaderMap) -> String {
        format!("Hello {}", headers["name"].to_str().unwrap())
    }

    let app = Router::new().route("/query", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/query").add_header("name", "Rizki").await;
    response.assert_status_ok();
    response.assert_text("Hello Rizki");
}

#[tokio::test]
async fn test_path_parameter() {
    async fn route(Path((id, id_category)): Path<(String, String)>) -> String {
        format!("Product {}, Category {}", id, id_category)
    }

    let app = Router::new().route("/products/{id}/categories/{id_category}", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/products/1/categories/2").await;
    response.assert_status_ok();
    response.assert_text("Product 1, Category 2");
}

#[tokio::test]
async fn test_body_string() {
    async fn route(body: String) -> String {
        format!("Body {}", body)
    }

    let app = Router::new().route("/post", post(route));

    let server = TestServer::new(app).unwrap();

    let response = server.post("/post").text("This is body").await;
    response.assert_status_ok();
    response.assert_text("Body This is body");
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    username: String,
    password: String,
}

#[tokio::test]
async fn test_body_json() {
    async fn route(payload: Result<Json<LoginRequest>, JsonRejection>) -> String {
        match payload {
            Ok(request) => format!("Hello {}", request.username),
            Err(err) => format!("Error: {:?}", err),
        }
    }

    let app = Router::new().route("/post", post(route));

    let server = TestServer::new(app).unwrap();

    let request = LoginRequest {
        username: "rizki".to_string(),
        password: "rahasia".to_string(),
    };

    let response = server.post("/post").json(&request).await;
    response.assert_status_ok();
    response.assert_text("Hello rizki");

    let response = server.post("/post").text("tidak valid").await;
    response.assert_status_ok();
    response.assert_text("Error: MissingJsonContentType(MissingJsonContentType)");
}

#[tokio::test]
async fn test_response() {
    async fn route(request: Request) -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("X-Owner", "Rizki")
            .body(Body::from(format!("Hello {}", request.method())))
            .unwrap()
    }

    let app = Router::new().route("/get", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");
    response.assert_header("X-Owner", "Rizki");
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    token: String,
}

#[tokio::test]
async fn test_response_json() {
    async fn route() -> Json<LoginResponse> {
        Json(LoginResponse {
            token: "TOKEN".to_string(),
        })
    }

    let app = Router::new().route("/get", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text_contains("TOKEN");
}

#[tokio::test]
async fn test_response_tuple() {
    async fn route() -> (StatusCode, HeaderMap, Json<LoginResponse>) {
        let mut header = HeaderMap::new();
        header.insert("X-Owner", HeaderValue::from_str("Rizki").unwrap());

        (
            StatusCode::OK,
            header,
            Json(LoginResponse {
                token: "TOKEN".to_string(),
            }),
        )
    }

    let app = Router::new().route("/get", get(route));

    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("{\"token\":\"TOKEN\"}");
    response.assert_header("X-Owner", "Rizki");
}

#[tokio::test]
async fn test_form_request() {
    async fn route(Form(form): Form<LoginRequest>) -> String {
        format!("Hello {}", form.username)
    }

    let app = Router::new().route("/post", post(route));

    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/post")
        .form(&{
            LoginRequest {
                username: "rizki".to_string(),
                password: "password".to_string(),
            }
        })
        .await;
    response.assert_status_ok();
    response.assert_text("Hello rizki");
}

#[tokio::test]
async fn test_multipart() {
    async fn route(mut payload: Multipart) -> String {
        let mut profile = Bytes::new();
        let mut username = "".to_string();

        while let Some(field) = payload.next_field().await.unwrap() {
            if field.name().unwrap_or("") == "profile" {
                profile = field.bytes().await.unwrap();
            } else if field.name().unwrap_or("") == "username" {
                username = field.text().await.unwrap()
            }
        }

        assert!(profile.len() > 0);
        format!("Hello {}", username)
    }

    let app = Router::new().route("/post", post(route));
    let server = TestServer::new(app).unwrap();

    let request = MultipartForm::new()
        .add_text("username", "rizki")
        .add_text("password", "rahasia")
        .add_part("profile", Part::bytes(Bytes::from("profile")));

    let response = server.post("/post").multipart(request).await;
    response.assert_status_ok();
    response.assert_text("Hello rizki");
}

#[tokio::test]
async fn test_cookie_response() {
    async fn route(query: Query<HashMap<String, String>>) -> (CookieJar, String) {
        let name = query.get("name").unwrap();
        (
            CookieJar::new().add(Cookie::new("name", name.clone())),
            format!("Hello {}", name.clone()),
        )
    }

    let app = Router::new().route("/query", get(route));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/query").add_query_param("name", "rizki").await;
    response.assert_status_ok();
    response.assert_text("Hello rizki");
    response.assert_contains_header("Set-Cookie");
    response.assert_header("Set-Cookie", "name=rizki");
}

#[tokio::test]
async fn test_cookie_request() {
    async fn route(cookies: CookieJar) -> String {
        format!("Hello {}", cookies.get("name").unwrap().value())
    }

    let app = Router::new().route("/cookie", get(route));
    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/cookie")
        .add_header("Cookie", "name=rizki")
        .await;
    response.assert_status_ok();
    response.assert_text("Hello rizki");
}

async fn log_middleware(request: Request, next: Next) -> Response {
    println!("Receive request {} {}", request.method(), request.uri());

    let response = next.run(request).await;
    println!("Send response {}", response.status());

    response
}

async fn request_id_middleware<T>(mut request: Request<T>) -> Request<T> {
    let request_id = "random-id";

    request
        .headers_mut()
        .insert("X-Request-ID", request_id.parse().unwrap());

    request
}

#[tokio::test]
async fn test_middleware() {
    async fn route(method: Method, header_map: HeaderMap) -> String {
        let request_id = header_map.get("X-Request-Id").unwrap().to_str().unwrap();
        format!("Hello {} {}", method, request_id)
    }

    let app = Router::new()
        .route("/get", get(route))
        .layer(map_request(request_id_middleware))
        .layer(from_fn(log_middleware));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Hello GET random-id");
}

struct AppError {
    code: i32,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.code as u16).unwrap(),
            self.message,
        )
            .into_response()
    }
}

#[tokio::test]
async fn test_error_handling() {
    async fn route(method: Method) -> Result<String, AppError> {
        if method == Method::POST {
            Ok("Ok".to_string())
        } else {
            Err(AppError {
                code: 400,
                message: "Ups error".to_string(),
            })
        }
    }

    let app = Router::new()
        .route("/get", get(route))
        .route("/post", post(route));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_bad_request();
    response.assert_text("Ups error");

    let response = server.post("/post").await;
    response.assert_status_ok();
    response.assert_text("Ok");
}

#[tokio::test]
async fn test_unexpected_error() {
    async fn route(request: Request) -> Result<Response, anyhow::Error> {
        if request.method() == Method::POST {
            Ok(Response::new(Body::empty()))
        } else {
            Err(anyhow!("Method is not allowed"))
        }
    }

    let route_service = tower::service_fn(route);

    async fn handle_error(err: anyhow::Error) -> (StatusCode, String) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error : {}", err),
        )
    }

    let app = Router::new().route_service("/get", HandleError::new(route_service, handle_error));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_internal_server_error();
    response.assert_text("Error : Method is not allowed");
}

struct DatabaseConfig {
    total: i32,
}

#[tokio::test]
async fn test_state_extractor() {
    let database_state = Arc::new(DatabaseConfig { total: 100 });

    async fn route(State(database): State<Arc<DatabaseConfig>>) -> String {
        format!("Total {}", database.total)
    }

    let app = Router::new()
        .route("/get", get(route))
        .with_state(database_state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Total 100");
}

#[tokio::test]
async fn test_state_extension() {
    let database_state = Arc::new(DatabaseConfig { total: 100 });

    async fn route(Extension(database): Extension<Arc<DatabaseConfig>>) -> String {
        format!("Total {}", database.total)
    }

    let app = Router::new()
        .route("/get", get(route))
        .layer(Extension(database_state));
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Total 100");
}

#[tokio::test]
async fn test_state_closure_capture() {
    let database_state = Arc::new(DatabaseConfig { total: 100 });

    async fn route(database: Arc<DatabaseConfig>) -> String {
        format!("Total {}", database.total)
    }

    let app = Router::new().route(
        "/get",
        get({
            let database_state = Arc::clone(&database_state);
            move || route(database_state)
        }),
    );
    let server = TestServer::new(app).unwrap();

    let response = server.get("/get").await;
    response.assert_status_ok();
    response.assert_text("Total 100");
}

#[tokio::test]
async fn test_multiple_router() {
    async fn route(method: Method) -> String {
        format!("Hello {}", method)
    }

    let first = Router::new().route("/first", get(route));
    let second = Router::new().route("/second", post(route));

    let app = Router::new().merge(first).merge(second);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/first").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");

    let response = server.post("/second").await;
    response.assert_status_ok();
    response.assert_text("Hello POST");
}

#[tokio::test]
async fn test_nest_multiple_router() {
    async fn route(method: Method) -> String {
        format!("Hello {}", method)
    }

    let first = Router::new().route("/first", get(route));
    let second = Router::new().route("/second", post(route));

    let app = Router::new()
        .nest("/api/users", first)
        .nest("/api/products", second);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/users/first").await;
    response.assert_status_ok();
    response.assert_text("Hello GET");

    let response = server.post("/api/products/second").await;
    response.assert_status_ok();
    response.assert_text("Hello POST");
}

#[tokio::test]
async fn test_fallback() {
    async fn route(method: Method) -> String {
        format!("Hello {}", method)
    }

    let first = Router::new().route("/first", get(route));
    let second = Router::new().route("/second", post(route));

    async fn fallback(request: Request) -> (StatusCode, String) {
        (
            StatusCode::NOT_FOUND,
            format!("Page {} is not found", request.uri().path()),
        )
    }

    let app = Router::new().merge(first).merge(second).fallback(fallback);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/wrong").await;
    response.assert_status_not_found();
    response.assert_text("Page /wrong is not found");
}

#[tokio::test]
async fn test_fallback_method_not_allowed() {
    async fn route(method: Method) -> String {
        format!("Hello {}", method)
    }

    let first = Router::new().route("/first", get(route));
    let second = Router::new().route("/second", post(route));

    async fn fallback(request: Request) -> (StatusCode, String) {
        (
            StatusCode::NOT_FOUND,
            format!("Page {} is not found", request.uri().path()),
        )
    }

    async fn not_allowed(request: Request) -> (StatusCode, String) {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            format!(
                "Method {} in {} is not allowed",
                request.method(),
                request.uri().path()
            ),
        )
    }

    let app = Router::new()
        .merge(first)
        .merge(second)
        .fallback(fallback)
        .method_not_allowed_fallback(not_allowed);
    let server = TestServer::new(app).unwrap();

    let response = server.post("/first").await;
    response.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    response.assert_text("Method POST in /first is not allowed");
}
