use axum::{ routing::get , Router };


pub async fn hello_from_the_server() -> &'static str {
    "Hello!"
}


pub fn init() -> Router<()> {
    Router::new()
        .route( "/hello" , get( hello_from_the_server ) )
}
