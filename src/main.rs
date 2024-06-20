use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let router = Router::new().route("/", get(index)).nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    );

    info!("starting server...");

    let port = 9999_u16;
    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();

    info!("server started on port {}", port);
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

/// Wrapper type to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows converting Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", error),
            )
                .into_response(),
        }
    }
}
